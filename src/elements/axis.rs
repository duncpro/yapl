// # Axis

#[derive(Clone, Copy)]
pub struct Axis<'a> {
    pub offset: f64,
    pub stride: f64,
    pub pos: f64,
    pub tick_label: Option<TickLabel<'a>>,
    pub tick: Tick<'a>,
    pub apply_default_style_class: bool,
    
    /// List of names of custom CSS styles classes to include in the `class` attibute
    /// of the `line` element. 
    ///
    /// Note that these classes are in addition to the default style class name, unless
    /// of course the default style class name has been explicitly omitted by setting
    /// `apply_default_style_class` to false.
    pub style_class: Option<&'a str>,   
}

impl<'a> Axis<'a> {
    pub fn new_default(offset: f64, stride: f64, pos: f64) -> Self {
        Self { 
            offset, stride, pos,
            tick_label: Some(TickLabel::new_default(TickLabelKind::Decimal)),
            tick: Tick::new_default(),
            apply_default_style_class: true,
            style_class: None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AxisDefaultStyleClass {
    pub apply_stroke_width: bool,
    pub apply_stroke: bool
}

impl AxisDefaultStyleClass {
    pub const ENABLED: Self = Self {
        apply_stroke_width: true,
        apply_stroke:       true
    };
    
    pub const DISABLED: Self = Self {
        apply_stroke_width: false,
        apply_stroke:       false
    };

    pub const NAME: &'static str = "yapl-def-axis";
}

pub const DEFAULT_AXIS_STROKE_WIDTH: f64 = 1.0 / 1000.0;
pub const DEFAULT_AXIS_STROKE: &'static str = "black";

pub(crate) fn write_axis_default_style_class(out: &mut impl std::io::Write, class: &AxisDefaultStyleClass)
-> std::io::Result<()>
{
    if class == &AxisDefaultStyleClass::DISABLED { return Ok(()); };
    write!(out, ".{} {{", AxisDefaultStyleClass::NAME)?;
    if class.apply_stroke_width {
        write!(out, "stroke-width: {};", DEFAULT_AXIS_STROKE_WIDTH)?;
    }
    if class.apply_stroke {
        write!(out, "stroke: {};", DEFAULT_AXIS_STROKE)?;
    }
    write!(out, "}}")?;
    return Ok(());
}

// # Tick

#[derive(Clone, Copy)]
pub struct Tick<'a> {
    pub len: f64,    
    pub apply_default_style_class: bool,
    
    /// List of names of custom CSS styles classes to include in the `class` attibute
    /// of the `line` element. 
    ///
    /// Note that these classes are in addition to the default style class name, unless
    /// of course the default style class name has been explicitly omitted by setting
    /// `apply_default_style_class` to false.
    pub style_class: Option<&'a str>,  
}


impl<'a> Tick<'a> {
    pub const DEFAULT_LEN: f64 = 1.0 / 100.0;
    
    pub fn new_default() -> Self {
        Self {
            len: Self::DEFAULT_LEN,
            apply_default_style_class: true,
            style_class: None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TickDefaultStyleClass {
    pub apply_stroke_width: bool,
    pub apply_stroke: bool
}


impl TickDefaultStyleClass {
    pub const ENABLED: Self = Self {
        apply_stroke_width: true,
        apply_stroke:       true,
    };
    
    pub const DISABLED: Self = Self {
        apply_stroke_width: false,
        apply_stroke:       false,
    };

    pub const NAME: &'static str = "yapl-def-tick";
}

pub const DEFAULT_TICK_STROKE_WIDTH: f64 = DEFAULT_AXIS_STROKE_WIDTH;
pub const DEFAULT_TICK_STROKE: &'static str = "black";

pub(crate) fn write_tick_default_style_class(out: &mut impl std::io::Write, class: &TickDefaultStyleClass)
-> std::io::Result<()>
{
    if class == &TickDefaultStyleClass::DISABLED { return Ok(()); };
    write!(out, ".{} {{", TickDefaultStyleClass::NAME)?;
    if class.apply_stroke_width {
        write!(out, "stroke-width: {};", DEFAULT_TICK_STROKE_WIDTH)?;
    }
    if class.apply_stroke {
        write!(out, "stroke: {};", DEFAULT_TICK_STROKE)?;
    }
    write!(out, "}}")?;
    return Ok(());
}


// # TickLabel

#[derive(Clone, Copy)]
pub struct TickLabel<'a> {
    pub kind: TickLabelKind<'a>,
    pub typography_height: f64
}

impl<'a> TickLabel<'a> {
    pub const DEFAULT_TYPOGRAPHY_HEIGHT: f64 = crate::typography::DEFAULT_TYPOGRAPHY_HEIGHT;
    
    pub fn new_default(kind: TickLabelKind<'a>) -> Self {
        Self {
            kind,
            typography_height: Self::DEFAULT_TYPOGRAPHY_HEIGHT,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TickLabelKind<'a> {
    Decimal,
    Symbolic(SymbolicTickLabel<'a>)
}

#[derive(Clone, Copy)]
pub struct SymbolicTickLabel<'a> {
    /// This field should always hold `Some` when the offset is set to a non-zero number
    /// in the axis definition, otherwise the labels will be misleading. However if this
    /// advice is not followed, no error will occur and the axis will still be labeled
    /// albeit inaccurately.
    ///
    /// Note: If the offset is set to zero in the axis definition, this symbol will
    /// still be rendered.
    pub offset_symbol_tex: Option<&'a str>,
    pub stride_symbol_tex: &'a str
}
