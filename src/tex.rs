pub trait TexRenderer {
    pub fn render(destin: impl std::io::Write, tex: &str) -> std::io::Result<()>;
}
