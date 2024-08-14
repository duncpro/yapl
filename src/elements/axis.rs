#[derive(Clone)]
pub struct SymbolicTickLabel {
    /// This field should always hold `Some` when the offset is set to a non-zero number
    /// in the axis definition, otherwise the labels will be misleading. However if this
    /// advice is not followed, no error will occur and the axis will still be labeled
    /// albeit inaccurately.
    ///
    /// Note: If the offset is set to zero in the axis definition, this symbol will
    /// still be rendered.
    pub offset_symbol_tex: Option<String>,
    pub stride_symbol_tex: String
}

#[derive(Clone)]
pub enum TickLabel {
    Decimal,
    Symbolic(SymbolicTickLabel)
}

#[derive(Clone)]
pub struct Axis {
    pub offset: f64,
    pub stride: f64,
    pub pos: f64,
    pub tick_appearance_length: f64,
    pub tick_label: Option<TickLabel>
}

impl Axis {
    pub const DEFAULT_TICK_APPEARANCE_LENGTH: f64 = 1.0 / 100.0;

    /// Creates a new [`Axis`] with the default `tick_appearance_length`.
    pub fn new_default(offset: f64, stride: f64, pos: f64) -> Self {
        Self { 
            offset, stride, pos,
            tick_appearance_length: Self::DEFAULT_TICK_APPEARANCE_LENGTH,
            tick_label: Some(TickLabel::Decimal)
        }
    }
}
