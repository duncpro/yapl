use yapl3::codegen_svg::{codegen_svg_cplane, DefaultSVGGlobalStyles};
use yapl3::elements::{CoordinatePlane, Function, FunctionKind};
use yapl3::math::{ClosedInterval, NonDecreasing};

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_minimal();
    cplane.extent.y = ClosedInterval::new(NonDecreasing::new(2.0, 10.0));
    cplane.extent.x = ClosedInterval::new(NonDecreasing::new(-10.0, 10.0));

    let mut f = Function::new_default(|x| x.powi(2));
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

