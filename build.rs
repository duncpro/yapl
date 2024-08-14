use std::process::Command;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut mathjax_dir = PathBuf::new();
    mathjax_dir.push(env!("CARGO_MANIFEST_DIR"));
    mathjax_dir.push("mathjax-wrapper");
    let success = Command::new("npm")
        .args(&["install"])
        .current_dir(mathjax_dir)
        .status()?
        .success();
    assert!(success);
    return Ok(())
}

