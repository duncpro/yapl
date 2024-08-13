use yapl3::codegen_svg::{codegen_svg_cplane, DefaultSVGGlobalStyles};
use yapl3::elements::{CoordinatePlane, Function};
use yapl3::math::{ClosedInterval, NonDecreasing};

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_minimal();
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.1, 1.1));
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-0.5, 0.5));
    cplane.extent.x_scale = 10.0;

    let mut f = Function::new_default(|x| (1.0 / x).sin());
    f.zero_tolerance_factor = 10.0f32.powi(7);
    cplane.fns.push(f);
      
    let mut out = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out.svg")?;

    let global_styles = DefaultSVGGlobalStyles::new();    
    codegen_svg_cplane(&mut out, &cplane, &global_styles)?;
    
    println!("Done");
    return Ok(())   
}
