use yapl::elements::{CoordinatePlane, Function};
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::style::Stylesheet;
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;

#[test]
fn test_1oversinx() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_minimal();
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-0.5, 0.5));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.1, 1.1));
    cplane.extent.x_scale = 8.0;

    let mut f = Function::new_elementary(|x| (1.0 / x).sin());
    f.zero_tolerance_factor = 10.0f64.powi(7);
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
    expectation_path.push("sin1overx.svg");
    
    let expectation = std::io::read_to_string(std::fs::File::open(expectation_path)?)?;
    let actual = std::io::read_to_string(std::fs::File::open(&actual_path)?)?;
    assert!(expectation == actual);
    
    return Ok(())   
}
