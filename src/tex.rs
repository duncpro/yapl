pub trait TexRenderer {
    fn render(destin: impl std::io::Write, tex: &str) -> std::io::Result<()>;
}
