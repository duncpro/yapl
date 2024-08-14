pub trait TeXRenderer {
    /// Renders the given TeX source string into SVG and writes it to `destin`.
    fn render(
        &self, 
        tex_src: &mut impl std::io::Read, 
        svg_destin: &mut impl std::io::Write,
        preserve_aspect_ratio: Option<&'static str>,
    )
    -> std::io::Result<()>;

    fn render_str(
        &self, 
        tex_str: &str, 
        html_destin: &mut impl std::io::Write,
        preserve_aspect_ratio: Option<&'static str>
    )
    -> std::io::Result<()>
    {
        let mut cursor = std::io::Cursor::new(tex_str);
        self.render(&mut cursor, html_destin, preserve_aspect_ratio)
    }

    fn render_num(
        &self, 
        num: f64, 
        html_destin: &mut impl std::io::Write,        
        preserve_aspect_ratio: Option<&'static str>
    )
    -> std::io::Result<()>
    {
        self.render_str(&num.to_string(), html_destin, preserve_aspect_ratio)
    }

}

// TODO: MathJaxProcessTexRenderer should keep the node process alive for the lifetime
//       of the struct to avoid paying the cost of node VM startup every time some text
//       is to be rendered.

// TODO: MathJaxProcessTexRenderer should implement render_num so as not to allocate
//       every time a number is typeset, like the default implementation does.

pub struct MathJaxProcessTexRenderer { entrypoint: String }

impl MathJaxProcessTexRenderer {
    pub fn new(entrypoint: String) -> Self { Self { entrypoint } }
}

impl TeXRenderer for MathJaxProcessTexRenderer {
    fn render(&self, tex_src: &mut impl std::io::Read, svg_destin: &mut impl std::io::Write,
        preserve_aspect_ratio: Option<&'static str>)
    -> std::io::Result<()> 
    {
        let mut command = std::process::Command::new("node");
        command.arg(&self.entrypoint);
        if let Some(arg) = preserve_aspect_ratio { command.arg(arg); }
        command.stdout(std::process::Stdio::piped());
        command.stdin(std::process::Stdio::piped());
        let mut process = command.spawn()?;
        std::io::copy(tex_src, &mut process.stdin.take().unwrap())?;
        std::io::copy(&mut process.stdout.take().unwrap(), svg_destin)?;
        return Ok(())
    }
}
