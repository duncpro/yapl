use std::f64::consts::PI;
use yapl::elements::{Function, CoordinatePlane, Axis, TickLabelKind, SymbolicTickLabel, TickLabel};
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;
use yapl::style::Stylesheet;

#[test]
fn test_sinx() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();
    let mut x_axis = Axis::new_default(0.0, PI, 0.0);
    x_axis.tick_label = Some(TickLabel::new_default(TickLabelKind::Symbolic(SymbolicTickLabel {
        offset_symbol_tex: None,
        stride_symbol_tex: "\\pi",
    })));
    cplane.horizontal_axis = Some(x_axis);
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-2.0 * PI - 1.0, 2.0 * PI + 1.0));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.5, 1.5));

    let f = Function::new_elementary(|x| x.sin());
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
    expectation_path.push("sinx.svg");
    
    let expectation = std::io::read_to_string(std::fs::File::open(expectation_path)?)?;
    let actual = std::io::read_to_string(std::fs::File::open(&actual_path)?)?;
    assert!(expectation == actual);
    
    return Ok(())   
}
