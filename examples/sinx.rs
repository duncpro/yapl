use std::f64::consts::PI;
use std::io::BufWriter;
use yapl::elements::{Function, CoordinatePlane, Axis, TickLabelKind, SymbolicTickLabel, TickLabel};
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;
use yapl::style::Stylesheet;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();
    let mut x_axis = Axis::new_default(0.0, PI, 0.0);
    x_axis.tick_label = Some(TickLabel::new_default(TickLabelKind::Symbolic(SymbolicTickLabel {
        offset_symbol_tex: None,
        stride_symbol_tex: "\\pi",
    })));
    cplane.horizontal_axis = Some(x_axis);
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-2.0 * PI - 1.0, 2.0 * PI + 1.0));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.5, 1.5));

    let f = Function::new_default(|x| x.sin());
    cplane.fns.push(f);

    let mut out_path = std::path::PathBuf::new();
    out_path.push(env!("CARGO_MANIFEST_DIR"));
    out_path.push("examples");
    out_path.push("sinx.svg");
    let file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_path)?;
    let mut out = BufWriter::new(file); 
        
    let stylesheet = Stylesheet::new_default();
    let mut tex_renderer = MathJaxProcessTeXRenderer::new()?;
    codegen(&mut out, &cplane, stylesheet, &mut tex_renderer)?;
    return Ok(())   
}
