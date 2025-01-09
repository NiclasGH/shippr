use std::process::Command;

fn main() {
    let output = Command::new("sudo")
        .arg("whoami")
        .output()
        .expect("Failed to execute process");
    let output_string = String::from_utf8(output.stdout).unwrap();

    println!("{}", output_string)
}
