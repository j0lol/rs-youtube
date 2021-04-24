use std::process::Command;

pub fn clear_screen() {

    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //         .args(&["/C", "cls"])
    //         .output()
    //         .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
    // } else {
    //     Command::new("clear")
    //         .output()
    //         .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
    // };
    //
    // print!("{}", String::from_utf8_lossy(&output.stdout));

    // Clears terminal AND scroll-back,
    // don't know how to do this on mac/win
    if cfg!(target_os = "linux") {
        let output = Command::new("clear")
                .output()
                .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        let term = console::Term::stdout();
        term.clear_screen().expect("failed clearing screen");
    }

}
