use yapl::elements::{Function, CoordinatePlane};
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;
use yapl::style::Stylesheet;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();

    let f = Function::new_elementary(|x| 1.0f64 / x);
    cplane.fns.push(f);

    let mut out_path = std::path::PathBuf::new();
    out_path.push(env!("CARGO_MANIFEST_DIR"));
    out_path.push("examples");
    out_path.push("1overx.svg");
    let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_path)?;
        
    let stylesheet = Stylesheet::new_default();
    let mut tex_renderer = MathJaxProcessTeXRenderer::new()?;
    codegen(&mut out, &cplane, stylesheet, &mut tex_renderer)?;
    return Ok(())   
}
