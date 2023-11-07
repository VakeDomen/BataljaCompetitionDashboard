use std::fs;
use std::io::{Error, ErrorKind, BufReader, BufRead};
use std::path::Path;
use std::process::{Command, Stdio};


pub fn execute_command(command: String, args: Vec<&str>) -> std::io::Result<Vec<String>> {
    let mut child = Command::new(command)
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().ok_or_else(|| {
        Error::new(ErrorKind::Other, "Could not capture standard output.")
    })?;
    
    let reader = BufReader::new(stdout);
    let output_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // Wait for the process to finish and check the exit status
    let status = child.wait()?;
    if !status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Command executed with non-zero exit status: {}", status),
        ));
    }

    Ok(output_lines)
}


pub fn recursive_copy(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !src.is_dir() {
        fs::copy(src, dest)?;
    } else {
        fs::create_dir_all(dest)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_child = dest.join(path.file_name().unwrap());
            recursive_copy(&path, &dest_child)?;
        }
    }
    Ok(())
}