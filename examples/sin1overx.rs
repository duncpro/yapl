use yapl::elements::{CoordinatePlane, Function};
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::style::Stylesheet;
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_minimal();
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-0.5, 0.5));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.1, 1.1));
    cplane.extent.x_scale = 8.0;

    let f = Function::new_elementary(|x| (1.0 / x).sin());
    cplane.fns.push(f);

    let mut out_path = std::path::PathBuf::new();
    out_path.push(env!("CARGO_MANIFEST_DIR"));
    out_path.push("examples");
    out_path.push("sin1overx.svg");
    let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_path)?;
       
    let mut tex_renderer = MathJaxProcessTeXRenderer::new()?;
    let stylesheet = Stylesheet::new_default();
    codegen(&mut out, &cplane, stylesheet, &mut tex_renderer)?;
    return Ok(())   
}
