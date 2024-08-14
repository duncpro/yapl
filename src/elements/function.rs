pub enum FunctionKind { OfX, OfY }

pub struct Function {
    pub eval: Box<dyn Fn(f64) -> f64>,

    /// See `min_depth` in [`crate::plotfn::PlotFnParams`].
    pub min_depth: usize,

    /// See `error_tolerance` in [`crate::plotfn::PlotFnParams`].
    ///
    /// Specifically `error_tolerance = codomain_length / error_tolerance_factor`.
    pub error_tolerance_factor: f64,

    /// See `zero_tolerance` in [`crate::plotfn::PlotFnParams`]
    ///
    /// Specifically `zero_tolerance = domain_length / zero_tolerance_factor`.
    pub zero_tolerance_factor: f64,

    pub kind: FunctionKind
}

impl Function {
    pub const DEFAULT_MIN_DEPTH: usize = 4;
    /// Based on the average display width in physical pixels.
    pub const DEFAULT_ERROR_TOLERANCE_FACTOR: f64 = 2000.0;
    /// Based on the average display width in physical pixels.
    pub const DEFAULT_ZERO_TOLERANCE_FACTOR: f64 = 2000.0;
    pub const DEFAULT_KIND: FunctionKind = FunctionKind::OfX;

    pub fn new_default<F>(f: F) -> Self
    where F: Fn(f64) -> f64 + 'static
    {
        Function {
            eval: Box::new(f),
            min_depth: Self::DEFAULT_MIN_DEPTH,
            error_tolerance_factor: Self::DEFAULT_ERROR_TOLERANCE_FACTOR,
            zero_tolerance_factor: Self::DEFAULT_ZERO_TOLERANCE_FACTOR,
            kind: Self::DEFAULT_KIND
        }
    }
}
