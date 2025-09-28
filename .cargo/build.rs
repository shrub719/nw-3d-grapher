use std::process::Command;

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=assets/icon.png");
    let output = Command::new("nwlink")
        .args(&["png-nwi", "assets/icon.png", "target/icon.nwi"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}
