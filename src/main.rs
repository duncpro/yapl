use std::f64::consts::PI;
use yapl::codegen::{DefaultGlobalStyles, codegen_cplane};
use yapl::elements::{CoordinatePlane, Axis, TickLabel, SymbolicTickLabel};
use yapl::elements::Function;
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::typesetting::MathJaxProcessTexRenderer;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();
    let mut x_axis = Axis::new_default(0.0, PI, 0.0);
    x_axis.tick_label = Some(TickLabel::Symbolic(SymbolicTickLabel {
        offset_symbol_tex: None,
        stride_symbol_tex: "\\pi".to_string(),
    }));
    cplane.horizontal_axis = Some(x_axis);
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-2.0 * PI - 1.0, 2.0 * PI + 1.0));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.5, 1.5));

    let mut f = Function::new_default(|x| x.sin());
    cplane.fns.push(f);
      
    let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("out.svg")?;

    let global_styles = DefaultGlobalStyles::new();   
    let tex_renderer = MathJaxProcessTexRenderer::new("mathjax-wrapper/main.mjs".to_string());
    codegen_cplane(&mut out, &cplane, &global_styles, &tex_renderer)?;
    println!("Done");
    return Ok(())   
}
