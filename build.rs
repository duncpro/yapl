use std::process::Command;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut mathjax_dir = PathBuf::new();
    mathjax_dir.push(env!("CARGO_MANIFEST_DIR"));
    mathjax_dir.push("mathjax-wrapper");
    
    let install_success = Command::new("npm")
        .arg("install")
        .current_dir(mathjax_dir)
        .status()?
        .success();
    assert!(install_success);
    
    let tsc_success = Command::new("npx")
        .arg("tsc")
        .current_dir(mathjax_dir)
        .status()?
        .success();
    assert!(tsc_success);
    
    return Ok(())
}

