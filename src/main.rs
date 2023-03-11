use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
struct Config {
    entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
struct Entry {
    description: Option<String>,
    command: String,
}

fn file_path(file_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", "hylo", "simple-rofi-menu") {
        let mut file_path = proj_dirs.config_dir().to_path_buf();
        file_path.push(file_name);
        file_path.set_extension("toml");
        return Ok(file_path);
    }
    Err("path error")?
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = file_path("config")?;
    let toml_str = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&toml_str)?;
    println!("{:?}", config.entries);

    let (descriptions, commands): (Vec<String>, Vec<String>) = config
        .entries
        .iter()
        .enumerate()
        .map(|(i, c)| {
            (
                format!(
                    "{} - {}",
                    i + 1,
                    c.description.to_owned().unwrap_or(c.command.to_owned())
                ),
                c.command.to_owned(),
            )
        })
        .unzip();

    let rofi_input = format!("0 - Exit\n{}", descriptions.join("\n"));
    println!("{}", rofi_input);

    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg("")
        .arg("-only-match")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    std::thread::spawn(move || {
        stdin
            .write_all(rofi_input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    let (choice, _) = stdout.split_once(' ').expect("Invalid choice");

    let choice = choice.parse::<isize>()? - 1;

    if choice > -1 {
        let command = commands.get(choice as usize).unwrap();
        Command::new("sh").arg("-c").arg(command).spawn()?;
    }

    Ok(())
}
