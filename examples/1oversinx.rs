use yapl::codegen::{DefaultGlobalStyles, codegen_cplane};
use yapl::elements::CoordinatePlane;
use yapl::elements::Function;
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::typesetting::MathJaxProcessTexRenderer;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_minimal();
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-0.5, 0.5));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.1, 1.1));
    cplane.extent.x_scale = 8.0;

    let mut f = Function::new_default(|x| (1.0 / x).sin());
    f.zero_tolerance_factor = 10.0f64.powi(7);
    cplane.fns.push(f);
      
    let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
        .open("1oversinx.svg")?;
    
    let global_styles = DefaultGlobalStyles::new();   
    let tex_renderer = MathJaxProcessTexRenderer::new();
    codegen_cplane(&mut out, &cplane, &global_styles, &tex_renderer)?;
    return Ok(())   
}
