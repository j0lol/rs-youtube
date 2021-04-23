use std::process::Command;

pub fn clear_screen() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cls")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
    } else {
        Command::new("clear")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
    };

    print!("{}", String::from_utf8_lossy(&output.stdout));
}
