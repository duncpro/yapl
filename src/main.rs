use yapl3::codegen_svg::{codegen_svg_cplane, DefaultSVGGlobalStyles};
use yapl3::elements::{CoordinatePlane, Function, FunctionKind};
use yapl3::math::{ClosedInterval, NonDecreasing};

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();

    let mut f = Function::new_default(|x| x.powi(2));
    f.kind = FunctionKind::OfY;
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
