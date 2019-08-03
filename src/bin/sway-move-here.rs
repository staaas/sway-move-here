use std::process::Command;
use std::vec::Vec;

fn swaymsg(args: Vec<&str>) {
    let output = Command::new("swaymsg").args(args).output().expect("failed to execute process");
    let stdout = String::from_utf8_lossy(output.stdout.as_slice());
    print!("{}", stdout);
}

fn main() {
    swaymsg(vec!["-t", "get_outputs"]);
}
