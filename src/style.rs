use crate::elements::function::FunctionDefaultStyleClass;
use crate::elements::axis::AxisDefaultStyleClass;
use crate::elements::axis::TickDefaultStyleClass;

/// By default, Yapl includes a minimal CSS stylesheet with sensible defaults in every SVG.
/// However, rules in this stylesheet can be made redundant through injection of custom styles.
/// To avoid including the redundant overridden default stylesheet, one may configure the
/// [`DefaultGlobalStyleClasses`] instance used during compilation.
///
/// Each field in this struct corresponds to a CSS style class. The value of each field
/// is a `DefaultStyleClass` struct with boolean flags for each rule included in the
/// the style class. These rules can be disabled on a selective basis or the default
/// class can be disabled completely in which case no class definition will be emitted.
///
/// # Example: Disable Rule
/// ```rust
/// use yapl::style::DefaultGlobalStyleClasses;
/// let mut def_global_styles = DefaultGlobalStyleClasses::ENABLED;
/// def_global_styles.function.apply_stroke_width = false;
/// ```
///
/// # Example: Disable Class
/// ```rust
/// use yapl::style::DefaultGlobalStyleClasses;
/// use yapl::elements::function::FunctionDefaultStyleClass;
/// let mut def_global_styles = DefaultGlobalStyleClasses::ENABLED;
/// def_global_styles.function = FunctionDefaultStyleClass::DISABLED;
/// ```
///
/// # Example: Disable Stylesheet
/// ```rust
/// use yapl::style::DefaultGlobalStyleClasses;
/// let def_global_styles = DefaultGlobalStyleClasses::DISABLED;
/// ```
///
/// # Example: Disable Class for Single Element
/// ```rust
/// use yapl::elements::Function;
/// let mut function = Function::new_default(|x| x);
/// function.apply_default_style_class = false;
/// ```
///
/// Yapl will emit the minimal default stylesheet based on the style configuration declared by
/// the [`DefaultGlobalStyleClasses`].
///
/// However, the class names will still be included in the class attributes on the SVG elements
/// unelss explicitly disabled on a per-element-instance basis via `el.apply_default_style_class`.
/// Generally speaking it is useful to retain the default class names at least as they are useful
/// for targeting the elements in a custom stylesheet.
///
/// Note: If Yapl emits no elements associated with a default style class it will omit the default
/// style class from the default stylesheet automatically. For instance there will be no
/// style class definition [`FunctionDefaultStyleClass::NAME`] in the stylesheet if no function
/// plots were emitted. Therefore, the default style classes should not be forcibly added to
/// other elements via `E#style_class` because the compiler assumes the set of targets for the
/// default style classes is closed.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DefaultGlobalStyleClasses {
    pub function: FunctionDefaultStyleClass,
    pub axis:     AxisDefaultStyleClass,
    pub tick:     TickDefaultStyleClass
}

impl DefaultGlobalStyleClasses {
    pub const ENABLED: Self = Self {
        function: FunctionDefaultStyleClass::ENABLED,
        axis:     AxisDefaultStyleClass::    ENABLED,
        tick:     TickDefaultStyleClass::    ENABLED,
    };

    pub const DISABLED: Self = Self {
        function: FunctionDefaultStyleClass::DISABLED,
        axis:     AxisDefaultStyleClass::    DISABLED,
        tick:     TickDefaultStyleClass::    DISABLED,
    };
}

/// The CSS stylesheet to be included within the SVG.
///
/// If `defaults` is set to `DefaultGlobalStyleClasse::DISABLED` and `custom` is set to `None`
/// the `<style>` tag will be omitted entirely from the SVG.
#[derive(Clone, Copy)]
pub struct Stylesheet<'a> {
    /// The default CSS classes to include in the stylesheet.
    ///    
    /// To omit the default style classes set `defaults` to `DefaultGlobalStyleClasses::DISABLED`.
    pub defaults: DefaultGlobalStyleClasses,
    /// Custom CSS to be included alongside the default style classes in the stylesheet.
    ///
    /// To omit the default style classes set `defaults` to `DefaultGlobalStyleClasses::DISABLED`.
    pub custom: Option<&'a str>
}

impl<'a> Stylesheet<'a> {
    pub fn new_default() -> Self {
        Self {
            defaults: DefaultGlobalStyleClasses::ENABLED,
            custom: None
        }
    }
}
