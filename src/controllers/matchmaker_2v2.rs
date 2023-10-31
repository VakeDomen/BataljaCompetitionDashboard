use std::{path::Path, fs};
use rand::Rng;

use crate::{
    db::{
        operations_competition::get_competition_by_id, 
        operations_teams::get_teams_by_competition_id, 
        operations_bot::get_bot_by_id,
    }, 
    models::{
        team::Team, 
        errors::MatchMakerError, bot::Bot
    }};

use super::command_executor::execute_command;

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

    for match_pair in match_pairs.iter() {
        let out = run_match(&match_pair.0, &match_pair.1)?;
        println!("{:#?}", out);
    }

    
    Ok(match_pairs)
}

fn compile_team_bots(teams: Vec<Team>) -> Vec<Team> {
    let mut compiled_teams: Vec<Team> = vec![];

    for team in teams.into_iter() {
        if team.bot1.eq("") || team.bot2.eq("") {
            continue
        }
        
        let bot1 = match get_bot_by_id(team.bot1.clone()) {
            Ok(b) => b,
            Err(_) => continue,
        };

        let bot2 = match get_bot_by_id(team.bot2.clone()) {
            Ok(b) => b,
            Err(_) => continue,
        };
        
        if let Err(e) = compile_bot(bot1) {
            println!("ERRRR: {:#?}", e);
            continue
        }

        if let Err(e) = compile_bot(bot2) {
            println!("ERRRR: {:#?}", e);
            continue
        }

        compiled_teams.push(team);
    }

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
fn compile_bot(bot: Bot) -> Result<(), MatchMakerError> {
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


fn run_match(team1: &Team, team2: &Team) -> Result<Vec<String>, MatchMakerError> {
    match execute_command("ls".to_string(), vec![]) {
        Ok(out) => Ok(out),
        Err(e) => Err(MatchMakerError::IOError(e))
    }
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
