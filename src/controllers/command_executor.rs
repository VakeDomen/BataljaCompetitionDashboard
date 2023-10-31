use std::io::{Error, ErrorKind, BufReader, BufRead};
use std::process::{Command, Stdio};

pub fn execute_command(command: String, args: Vec<&str>) -> Result<Vec<String>, Error> {
    let process = match Command::new(command)
        .args(&args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn() 
    {
        Ok(p) => p,
        Err(e) => return Err(Error::new(
            ErrorKind::Other, 
            format!("Something went wrong with running command: {}", e.to_string()))
        ),
    };
    
    let stdout = process
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;
        
    let mut output_lines = vec![];
    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| output_lines.push(line));

    Ok(output_lines)
}
