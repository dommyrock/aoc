use std::process::Command;

fn main() {
    let output = Command::new("python")
        .arg("./src/bin/script.py")
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
