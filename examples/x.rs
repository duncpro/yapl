use yapl::elements::{Function, CoordinatePlane};
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;
use yapl::style::Stylesheet;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();

    let f = Function::new_elementary(|x| x);
    cplane.fns.push(f);

    let mut out_path = std::path::PathBuf::new();
    out_path.push(env!("CARGO_MANIFEST_DIR"));
    out_path.push("examples");
    out_path.push("x.svg");
    let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_path)?;
       
    let mut tex_renderer = MathJaxProcessTeXRenderer::new()?;
    let stylesheet = Stylesheet::new_default();
    codegen(&mut out, &cplane, stylesheet, &mut tex_renderer)?;
    
    return Ok(());
}
