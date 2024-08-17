use yapl::elements::{Function, CoordinatePlane};
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;
use yapl::style::Stylesheet;

#[test]
fn test_x() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();

    let f = Function::new_elementary(|x| x);
    cplane.fns.push(f);
    
    let mut actual_path = std::env::temp_dir();
    actual_path.push("yapl-actual.svg");

    {
        let mut out = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&actual_path)?;

        let mut tex_renderer = MathJaxProcessTeXRenderer::new()?;
        let stylesheet = Stylesheet::new_default();
        codegen(&mut out, &cplane, stylesheet, &mut tex_renderer)?;
    }

    let mut expectation_path = std::path::PathBuf::new();
    expectation_path.push(env!("CARGO_MANIFEST_DIR"));
    expectation_path.push("tests");
    expectation_path.push("x.svg");
    
    let expectation = std::io::read_to_string(std::fs::File::open(expectation_path)?)?;
    let actual = std::io::read_to_string(std::fs::File::open(&actual_path)?)?;
    assert!(expectation == actual);
    return Ok(())   
}
