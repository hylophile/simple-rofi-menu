use std::env;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
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

    let rofi_input = "0 - Exit
1 - Toggle movie mode
2 - Toggle virtual keyboard
3 - Toggle zoom
4 - mpv
5 - dolphin
6 - firefox
";

    std::thread::spawn(move || {
        stdin
            .write_all(rofi_input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    let (choice, _) = stdout.split_once(' ').expect("Invalid choice");

    let home = env::var("HOME")?;

    match choice {
        "0" => {
            println!("exit");
        }
        "1" => {
            Command::new("sh")
                .arg(format!("{home}/.config/waybar/scripts/toggle-dpms.sh"))
                .arg("toggle")
                .spawn()?;
            Command::new("sh")
                .arg(format!("{home}/.config/waybar/scripts/toggle-big.sh"))
                .spawn()?;
        }
        "2" => {
            Command::new("sh")
                .arg("-c")
                .arg("kill -s 34 $(pidof wvkbd-mobintl)")
                .spawn()?;
        }
        "3" => {
            Command::new("sh")
                .arg(format!("{home}/.config/waybar/scripts/toggle-big.sh"))
                .spawn()?;
        }
        "4" => {
            Command::new("mpv")
                .arg("--player-operation-mode=pseudo-gui")
                .spawn()?;
        }
        "5" => {
            Command::new("dolphin").spawn()?;
        }
        "6" => {
            Command::new("firefox-nightly").spawn()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
