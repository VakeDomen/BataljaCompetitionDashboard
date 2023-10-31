use std::{path::Path, fs, process::{Command, Stdio}, os::unix::process::CommandExt};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

use crate::{
    db::{
        operations_competition::{get_competition_by_id, set_competition_round}, 
        operations_teams::get_teams_by_competition_id, 
        operations_bot::{get_bot_by_id, set_bot_error}, operations_game2v2::insert_game,
    }, 
    models::{
        team::Team, 
        errors::MatchMakerError, 
        bot::Bot, 
        game_2v2::{NewGame2v2, Game2v2}, 
        competition::Competition
    }
};

use super::command_executor::{execute_command, recursive_copy};

/// Runs a 2v2 round for a specified competition.
///
/// This function manages the execution of a single 2v2 round for a competition, which includes:
/// 1. Fetching the competition details from the database.
/// 2. Retrieving all the teams participating in the competition.
/// 3. Compiling the bots for each team.
/// 4. Creating match pairs for the round.
/// 5. Running each match in parallel.
/// 6. Cleaning up the match directory after all games have been executed.
/// 7. Incrementing the competition round for the next set of matches.
///
/// # Arguments
///
/// * `competition_id` - A string representing the ID of the competition for which the round is to be run.
///
/// # Returns
///
/// A `Result` containing a `Vec` of tuples, where each tuple contains two teams that played against each other in the round. 
/// If successful, or a `MatchMakerError` if there's an error.
///
/// # Errors
///
/// This function will return an error if:
/// - The competition cannot be fetched from the database.
/// - The teams for the specified competition cannot be retrieved.
/// - There's an issue compiling the bots for any team.
/// - There's an error running any of the matches.
/// - The cleanup process fails.
/// - There's a problem updating the competition's round in the database.
///
pub fn run_2v2_round(competition_id: String) -> Result<Vec<(Team, Team)>, MatchMakerError> {
    let competition = match get_competition_by_id(competition_id) {
        Ok(c) => c,
        Err(e) => return Err(MatchMakerError::DatabaseError(e))
    };

    let teams = match get_teams_by_competition_id(competition.id.clone()) {
        Ok(teams) => teams,
        Err(e) => return Err(MatchMakerError::DatabaseError(e))
    };

    let compiled_teams = compile_team_bots(teams);
    let match_pairs = create_match_pairs(competition.games_per_round, compiled_teams);

    match_pairs.par_iter().for_each(|match_pair| {
        let result = run_match(&competition, &match_pair.0, &match_pair.1);
        
        match result {
            Ok(out) => println!("{:#?}", out),
            Err(e) => eprintln!("Error: {}", e), // Handle the error here or log it
        }
    });
  
    // Cleanup: Remove the match directory
    cleanup_matches()?;
    
    // increment competition round
    let new_round = competition.round + 1;
    if let Err(e) = set_competition_round(competition.id.clone(), new_round) {
        return Err(MatchMakerError::DatabaseError(e))
    }  

    Ok(match_pairs)
}

/// Cleans up the matches directory by removing all sub-directories.
///
/// This function is designed to remove all game-related folders that were 
/// created during individual matches within the `./resources/matches/` directory.
/// It ensures the top-level `matches` directory remains intact while all its
/// sub-directories (representing individual matches) are deleted.
///
/// # Returns
///
/// A `Result` which is `Ok(())` if the cleanup was successful, or a `MatchMakerError` 
/// if there's an error during the cleanup process.
///
fn cleanup_matches() -> Result<(), MatchMakerError> {
    // Cleanup: Remove all sub-directories within the ./resources/matches/ directory
    let matches_path = Path::new("./resources/matches");
    if let Ok(entries) = fs::read_dir(matches_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    if let Err(e) = fs::remove_dir_all(entry.path()) {
                        return Err(MatchMakerError::IOError(e));
                    }
                }
            }
        }
    }
    Ok(())
}


/// Runs a game match between two teams in a given competition.
///
/// This function manages the preparation, execution, and cleanup of a game match between two teams.
/// The steps include:
/// 1. Initializing a new 2v2 game instance based on the teams and competition details.
/// 2. Creating a unique directory for the match within the `./resources/matches` folder.
/// 3. Copying the bots of both teams to the match directory.
/// 4. Running the game using the Evaluator JAR, ensuring the game and its spawned bot processes 
///    are grouped together for easy management.
/// 5. Saving the game's output to a file within the `./resources/games` folder.
/// 6. Cleaning up by terminating any lingering processes related to the game to prevent zombies.
/// 7. Parsing the game output to produce a structured representation of the game results.
/// 8. Cleaning up by removing the match directory created in step 2.
///
/// # Arguments
///
/// * `competition` - A reference to the competition in which the teams are participating.
/// * `team1` - The first team participating in the match.
/// * `team2` - The second team participating in the match.
///
/// # Returns
///
/// A `Result` containing the structured game results (`Game2v2`) if successful. If there are any
/// issues during the preparation, execution, or cleanup, a `MatchMakerError` will be returned.
fn run_match(competition: &Competition, team1: &Team, team2: &Team) -> Result<Game2v2, MatchMakerError> {
    // Initialize a new 2v2 game with details from the provided teams and competition
    let match_game = NewGame2v2::new(
        competition.id.clone(),
        competition.round.to_string(),
        team1.id.clone(),
        team2.id.clone(),
        team1.bot1.clone(),
        team1.bot2.clone(),
        team2.bot1.clone(),
        team2.bot2.clone(),
    );

    // Create a directory to store match-related files
    let match_folder = Path::new("./resources/matches").join(match_game.id.to_string());
    if let Err(e) = fs::create_dir_all(&match_folder) {
        return Err(MatchMakerError::IOError(e));
    }

    // Copy each bot from the work directory to the match directory
    let bots = vec![&team1.bot1, &team1.bot2, &team2.bot1, &team2.bot2];
    for bot_id in &bots {
        let source = Path::new("./resources/workdir/bots").join(bot_id);
        let destination = match_folder.join(bot_id);
        
        if let Err(e) = recursive_copy(&source, &destination) {
            return Err(MatchMakerError::IOError(e));
        }
    }

    // Execute the game using the Evaluator JAR and collect the paths of each bot
    let mut bot_paths: Vec<String> = bots.iter().map(|bot_id| match_folder.join(bot_id).to_string_lossy().to_string()).collect();
    let output_file = format!("./resources/games/{}.txt", match_game.id.to_string());
    let mut command_args = vec![
        "-jar".to_string(),
        "resources/gamefiles/Evaluator.jar".to_string(),
    ];
    command_args.append(&mut bot_paths);


    // Run the game command and capture its output
    let result = Command::new("java")
        .args(&command_args)
        .stdout(Stdio::piped())
        .before_exec(|| {
            // Set the spawned process into its own process group
            unsafe {
                libc::setpgid(0, 0);
            }
            Ok(())
        })
        .spawn()
        .map_err(|e| MatchMakerError::IOError(e))?;

    let cid = result.id().clone();
    let output = result.wait_with_output().map_err(|e| MatchMakerError::IOError(e))?;


    // Ensure no zombies are left
    let pgid = unsafe { libc::getpgid(cid as libc::pid_t) };
    unsafe { libc::killpg(pgid, libc::SIGTERM) };

    // Save the game's output to the specified file
    if let Err(e) = fs::write(&output_file, &output.stdout) {
        return Err(MatchMakerError::IOError(e));
    }

    // Convert the game's output into a vector of strings
    let lines: Vec<String> = String::from_utf8_lossy(&output.stdout).lines().map(String::from).collect();

    // Parse the game using the provided function and return the result
    parse_game(lines, match_game)
}

/// Parses game output to determine match results and constructs a `Game2v2` object.
///
/// This function processes the output lines from a game match to extract relevant information
/// such as which bots survived and the scores of each bot. Based on this information, it 
/// determines the winner of the match and constructs a `Game2v2` object that encapsulates 
/// these details.
///
/// The function expects lines in the format `R <score> <color>` to determine scores of each bot. 
/// Colors (`red`, `blue`, `green`, `yellow`) are associated with bots from both teams.
///
/// # Arguments
///
/// * `lines` - A vector of strings representing the game's output lines.
/// * `match_game` - A mutable `NewGame2v2` object that contains initial game details and will be 
///                  updated with the parsed results.
///
/// # Returns
///
/// A `Result` containing a `Game2v2` object if successful, or a `MatchMakerError` if there's an error.
///
fn parse_game(lines: Vec<String>, mut match_game: NewGame2v2) -> Result<Game2v2, MatchMakerError> {
    let mut r_red = 0;
    let mut r_blue = 0;
    let mut r_green = 0;
    let mut r_yellow = 0;

    for line in lines.into_iter() {
        if line.contains("R ") {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() == 3 {
                match parts[2] {
                    "red"       => r_red    = parts[1].parse().unwrap_or(0),
                    "blue"      => r_blue   = parts[1].parse().unwrap_or(0),
                    "green"     => r_green  = parts[1].parse().unwrap_or(0),
                    "yellow"    => r_yellow = parts[1].parse().unwrap_or(0),
                    _ => ()
                }
            }
        }
    }

    match_game.team1bot1_survived = r_red > 0;
    match_game.team1bot2_survived = r_blue > 0;
    match_game.team2bot1_survived = r_green > 0;
    match_game.team2bot2_survived = r_yellow > 0;

    match (
        &match_game.team1bot1_survived,
        &match_game.team1bot2_survived,
        &match_game.team2bot1_survived,
        &match_game.team2bot2_survived
    ) {
        (true,  true,  false, false) => match_game.winner_id = match_game.team1_id.clone(),
        (true,  false, false, false) => match_game.winner_id = match_game.team1_id.clone(),
        (false, true,  false, false) => match_game.winner_id = match_game.team1_id.clone(),
        (false, false, true,  true)  => match_game.winner_id = match_game.team2_id.clone(),
        (false, false, true,  false) => match_game.winner_id = match_game.team2_id.clone(),
        (false, false, false, true)  => match_game.winner_id = match_game.team2_id.clone(),
        (_, _, _, _) => match_game.winner_id = "".to_string(),
    }

    if match_game.winner_id.eq("") {
        let t1_score = r_red + r_blue;
        let t2_score = r_green + r_yellow;
        if t1_score > t2_score {
            match_game.winner_id = match_game.team1_id.clone();
        } else {
            match_game.winner_id = match_game.team2_id.clone();
        }
    }

    match insert_game(match_game) {
        Ok(g) => Ok(g),
        Err(e) => Err(MatchMakerError::DatabaseError(e)),
    }
}



/// Attempts to compile the bots associated with each team in parallel.
///
/// This function performs the following steps for each team:
/// 1. If a team doesn't have both bot1 and bot2, the team is skipped.
/// 2. Retrieves the details of bot1 and bot2. If there's an error fetching the details, the team is skipped.
/// 3. Tries to compile bot1 and bot2. If there's a compilation error, an error is set for the respective bot.
/// 4. Teams with successful bot compilations are collected and returned.
///
/// # Arguments
///
/// * `teams` - A vector of `Team` objects for which bots need to be compiled.
///
/// # Returns
///
/// * A vector of `Team` objects for which both bots were successfully compiled.
///
/// # Notes
///
/// This function uses parallel processing for improved performance. Each team's bots are compiled in a separate thread.
///
fn compile_team_bots(teams: Vec<Team>) -> Vec<Team> {
    // Parallel processing of each team to compile associated bots
    let results: Vec<Result<Team, MatchMakerError>> = teams.into_par_iter().filter_map(|team| {
        // Skip teams without both bot1 and bot2
        if team.bot1.eq("") || team.bot2.eq("") {
            return None
        }

        // Retrieve bot details
        let bot1 = match get_bot_by_id(team.bot1.clone()) {
            Ok(b) => b,
            Err(e) => return Some(Err(MatchMakerError::DatabaseError(e))),
        };

        let bot2 = match get_bot_by_id(team.bot2.clone()) {
            Ok(b) => b,
            Err(e) => return Some(Err(MatchMakerError::DatabaseError(e))),
        };
        
        // Attempt to compile bot1
        if let Err(e) = compile_bot(&bot1) {
            if let Err(e) = set_bot_error(bot1, e.to_string()) {
                return Some(Err(MatchMakerError::DatabaseError(e)));
            }
            return Some(Err(e))
        }

        // Attempt to compile bot2
        if let Err(e) = compile_bot(&bot2) {
            if let Err(e) = set_bot_error(bot2, e.to_string()) {
                return Some(Err(MatchMakerError::DatabaseError(e)));
            }
            return Some(Err(e))
        }

        // Return the team if both bots compiled successfully
        Some(Ok(team))
    }).collect();

    // Extract teams with successful bot compilations
    let compiled_teams: Vec<Team> = results.into_iter().filter_map(|res| {
        match res {
            Ok(team) => Some(team),
            Err(_) => None,
        }
    }).collect();

    compiled_teams
}


/// Compiles the provided bot's source code.
///
/// This function performs the following tasks:
/// 1. Creates a working directory specific to the bot.
/// 2. Copies the bot's ZIP file to the working directory.
/// 3. Unzips the bot's ZIP file.
/// 4. Finds any Java files inside the unzipped directory.
/// 5. Compiles the Java files using the `javac` command.
///
/// # Arguments
///
/// * `bot` - A `Bot` instance containing the bot's details, including the source path.
///
/// # Returns
///
/// * `Ok(())` if the bot's source code was compiled successfully.
/// * `Err(MatchMakerError)` if any step in the process fails.
///
/// # Errors
///
/// This function will return an error if:
/// * The working directory cannot be created.
/// * The ZIP file cannot be copied or unzipped.
/// * No Java files are found in the unzipped directory.
/// * The Java files cannot be compiled.
/// 
fn compile_bot(bot: &Bot) -> Result<(), MatchMakerError> {
    let workdir = Path::new("./resources/workdir/bots").join(bot.id.clone());
    let source_path = Path::new(&bot.source_path);

    // Create a dedicated working directory for the bot.
    if let Err(e) = fs::create_dir_all(&workdir) {
        return Err(MatchMakerError::IOError(e));
    }

    // Convert the paths to string representations for command execution.
    let workdir_str = match workdir.as_os_str().to_str() {
        Some(s) => s,
        None => return Err(MatchMakerError::InvalidPath(workdir.into())),
    };
    let source_path_str = match source_path.as_os_str().to_str() {
        Some(s) => s,
        None => return Err(MatchMakerError::InvalidPath(source_path.into())),
    };

    // Copy the bot's ZIP file to its working directory.
    if let Err(e) = execute_command(
        "cp".to_string(), 
        vec![source_path_str, workdir_str]
    ) {
        return Err(MatchMakerError::IOError(e))
    };

    // Extract the file name from the source path.
    let file_name_osstr = match source_path.file_name() {
        Some(n) => n,
        None => return Err(MatchMakerError::InvalidPath(source_path.into())),
    };
    
    let file_name_str = match file_name_osstr.to_str() {
        Some(s) => s,
        None => return Err(MatchMakerError::InvalidPath(source_path.into())),
    };

    // Unzip the bot's ZIP file in the working directory.
    let unzip_target = workdir.join(file_name_str);
    let unzip_target_str = match unzip_target.as_os_str().to_str() {
        Some(s) => s,
        None => return Err(MatchMakerError::InvalidPath(unzip_target.into())),
    };
    
    if let Err(e) = execute_command(
        "unzip".to_string(), 
        vec!["-o", unzip_target_str, "-d", workdir_str]
    ) {
        return Err(MatchMakerError::IOError(e));
    }

    // Retrieve a list of Java files from the unzipped directory.
    let java_files: Vec<String> = match fs::read_dir(&workdir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension() == Some(std::ffi::OsStr::new("java")))
            .map(|entry| entry.path().display().to_string())
            .collect(),
        Err(e) => return Err(MatchMakerError::IOError(e))
    };
    
    if java_files.is_empty() {
        return Err(MatchMakerError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, "No Java files found")));
    }
    
    // Convert the list of file paths to a format suitable for the `javac` command.
    let java_files_str: Vec<&str> = java_files
        .iter()
        .map(AsRef::as_ref)
        .collect();

    // Compile the Java files.
    if let Err(e) = execute_command(
        "javac".to_string(),
        java_files_str
    ) {
        return Err(MatchMakerError::IOError(e));
    }

    Ok(())
}

/// Creates match pairs for a set of teams.
///
/// # Arguments
///
/// * `match_num` - The number of matches each team should play.
/// * `teams` - A vector containing all the teams.
///
/// # Returns
///
/// A vector containing tuples, where each tuple represents a match between two teams.
///
/// # Panics
///
/// The function may panic if the random number generation fails.
/// 
fn create_match_pairs(match_num: i32, teams: Vec<Team>) -> Vec<(Team, Team)> {
    let mut pairs = Vec::new();
    let games_to_play = ((teams.len() as f32 * match_num as f32) / 2.).ceil() as i32;

    let mut players: Vec<usize> = std::iter::repeat(0..teams.len())
        .take(match_num as usize)
        .flatten()
        .collect();

    while (pairs.len() as i32) < games_to_play {
        let random_index = rand::thread_rng().gen_range(0..players.len());
        let first_team_index = players.swap_remove(random_index);
    
        if players.len() < 1 {
            break
        }

        let random_index = rand::thread_rng().gen_range(0..players.len());
        let second_team_index = players.swap_remove(random_index);
    
        pairs.push((
            teams[first_team_index].clone(),
            teams[second_team_index].clone(),
        ));
    }

    pairs
}
