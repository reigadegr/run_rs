use std::io;
use std::process::{Command, Output};

pub fn run_command(command: String) -> io::Result<()> {
    let args: Vec<&str> = command.split_whitespace().collect();
    if let Some(cmd) = args.clone().first() {
        let args = args.into_iter().skip(1).collect::<Vec<&str>>();
        let mut child = Command::new("cmd")
            .args(&["/C", cmd])
            .args(&args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        let _ = child.wait()?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "No command provided"))
    }
}

pub fn run_command_no_msg(command: String) -> io::Result<Output> {
    Command::new("cmd").args(&["/C", &command]).output()
}