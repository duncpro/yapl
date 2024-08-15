pub enum FunctionKind { OfX, OfY }

pub struct Function<'a> {
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

    pub kind: FunctionKind,

    pub apply_default_style_class: bool,

    /// Space-delimited list of names of custom CSS styles classes to include in the `class`
    /// attibute of the `path` element. 
    ///
    /// Note that these classes are in addition to the default style class name, unless
    /// of course the default style class name has been explicitly omitted by setting
    /// `apply_default_style_class` to false.
    pub style_class: Option<&'a str>
}

impl<'a> Function<'a> {
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
            kind: Self::DEFAULT_KIND,
            apply_default_style_class: true,
            style_class: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FunctionDefaultStyleClass {
    pub apply_default_stroke_width: bool,
    pub apply_default_linecap: bool,
    pub apply_default_linejoin: bool,
    pub apply_default_fill: bool,
    pub apply_default_stroke: bool
}

impl FunctionDefaultStyleClass {

    pub const ENABLED: Self = Self {
        apply_default_stroke_width: true,
        apply_default_linecap:      true,
        apply_default_linejoin:     true,
        apply_default_fill:         true,
        apply_default_stroke:       true,
    };
    
    pub const DISABLED: Self = Self {
        apply_default_stroke_width: false,
        apply_default_linecap:      false,
        apply_default_linejoin:     false,
        apply_default_fill:         false,
        apply_default_stroke:       false,
    };

    pub const NAME: &'static str = "yapl-def-fn";
}

pub const DEFAULT_FUNCTION_STROKE_WIDTH: f64 = 1.0 / 400.0;
pub const DEFAULT_FUNCTION_LINECAP: &str = "round";
pub const DEFAULT_FUNCTION_LINEJOIN: &str = "round";
pub const DEFAULT_FUNCTION_FILL: &str = "none";
pub const DEFAULT_FUNCTION_STROKE: &str = "black";

pub(crate) fn write_function_default_style_class(out: &mut impl std::io::Write, 
    class: &FunctionDefaultStyleClass)
-> std::io::Result<()>
{
    if class == &FunctionDefaultStyleClass::DISABLED { return Ok(()) }
    write!(out, ".{} {{", FunctionDefaultStyleClass::NAME)?;
    if class.apply_default_stroke_width {
        write!(out, "stroke-width: {};", DEFAULT_FUNCTION_STROKE_WIDTH)?;
    }
    if class.apply_default_linecap {
        write!(out, "stroke-linecap: {};", DEFAULT_FUNCTION_LINECAP)?;
    }
    if class.apply_default_linejoin {
        write!(out, "stroke-linejoin: {};", DEFAULT_FUNCTION_LINEJOIN)?;
    }
    if class.apply_default_fill {
        write!(out, "fill: {};", DEFAULT_FUNCTION_FILL)?;
    }
    if class.apply_default_stroke {
        write!(out, "stroke: {};", DEFAULT_FUNCTION_STROKE)?;
    }
    write!(out, "}}")?;
    return Ok(());
}
