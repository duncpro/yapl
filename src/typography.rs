pub const DEFAULT_TYPOGRAPHY_HEIGHT: f64 = 2.0 / 100.0;

pub trait TeXRenderer {
    fn render_str(
        &mut self, 
        tex_str: &str, 
        html_destin: &mut impl std::io::Write,
        preserve_aspect_ratio: Option<&'static str>
    )
    -> std::io::Result<()>;

    fn render_num(
        &mut self, 
        num: f64, 
        html_destin: &mut impl std::io::Write,        
        preserve_aspect_ratio: Option<&'static str>
    )
    -> std::io::Result<()>
    {
        self.render_str(&num.to_string(), html_destin, preserve_aspect_ratio)
    }

    fn dump_css(&mut self, css_destin: &mut impl std::io::Write) -> std::io::Result<()>;
}

// TODO: MathJaxProcessTexRenderer should keep the node process alive for the lifetime
//       of the struct to avoid paying the cost of node VM startup every time some text
//       is to be rendered.

// TODO: MathJaxProcessTexRenderer should implement render_num so as not to allocate
//       every time a number is typeset, like the default implementation does.

use std::io::{Read, Write};
use crate::assert_matches;
use crate::misc::read_u32_le;

pub struct MathJaxProcessTeXRenderer {
    child_process: std::process::Child
}


impl MathJaxProcessTeXRenderer {
    pub fn new() -> std::io::Result<Self> {
        let mut path = std::path::PathBuf::new();
        path.push(env!("CARGO_MANIFEST_DIR"));
        path.push("mathjax-wrapper/target/main.mjs");

        let mut command = std::process::Command::new("node");
        // command.arg("--enable-source-maps");
        // command.arg("--trace-warnings");
        command.arg(path);
        command.stdout(std::process::Stdio::piped());
        command.stdin(std::process::Stdio::piped());
        let process = command.spawn()?;
        
        return Ok(Self { child_process: process });
    }
}

impl TeXRenderer for MathJaxProcessTeXRenderer {
    fn render_str(
        &mut self, 
        tex_str: &str, 
        svg_destin: &mut impl std::io::Write,
        preserve_aspect_ratio: Option<&'static str>,
    ) 
    -> std::io::Result<()> 
    {
        assert_matches!(&mut self.child_process.stdin, Some(child_stdin));
        
        // Recall the `ConversionRequest` packet begins with packet ID 0. 
        child_stdin.write(&0u32.to_le_bytes())?;
        // Now send the value of the `PreserveAspectRatioLength` field.
        let par_len = u32::try_from(preserve_aspect_ratio.map(|s| s.len()).unwrap_or(0)).unwrap();
        child_stdin.write(&par_len.to_le_bytes())?;
        // Now send the value of the `PreserveAspectRatio` field.
        if let Some(s) = &preserve_aspect_ratio {
            child_stdin.write(s.as_bytes())?;
        }
        // Now send the value of the `InputTeXLength` field.
        child_stdin.write(&u32::try_from(tex_str.len()).unwrap().to_le_bytes())?;
        // Now send the value of the `InputTeX` field.
        child_stdin.write(tex_str.as_bytes())?;

        child_stdin.flush()?;

        assert_matches!(&mut self.child_process.stdout, Some(child_stdout));

        // Now read the OutputSVGLength field.
        let svg_len = read_u32_le(child_stdout)?;

        // Now pipe the OutputSVG field.
        std::io::copy(&mut child_stdout.take(u64::from(svg_len)), svg_destin)?;
        
        svg_destin.flush()?;

        return Ok(());
    }

    fn dump_css(&mut self, css_destin: &mut impl std::io::Write) -> std::io::Result<()> {
        assert_matches!(&mut self.child_process.stdin, Some(child_stdin));
        
        // Recall the `StylesheetRequest` packet begins with packet id 1.
        child_stdin.write(&1u32.to_le_bytes())?;

        child_stdin.flush()?;

        assert_matches!(&mut self.child_process.stdout, Some(child_stdout));
                
        // Now read the OutputCSSLength field.
        let css_len = read_u32_le(child_stdout)?;
        
        // Now pipe the OutputCSS field.
        std::io::copy(&mut child_stdout.take(u64::from(css_len)), css_destin)?;

        css_destin.flush()?;
        
        return Ok(());
    }
}

impl Drop for MathJaxProcessTeXRenderer {
    fn drop(&mut self) {
        assert_matches!(&mut self.child_process.stdin, Some(child_stdin));
        // Recall the `ShutdownRequest` packet begins with packet id 2.
        child_stdin.write(&2u32.to_le_bytes()).unwrap();
        child_stdin.flush().unwrap();
        let status = self.child_process.wait().unwrap();
        assert!(status.success());
    }
}
