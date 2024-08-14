/// A generic non-decreasing interval of real numbers. This interval is neither open nor
/// closed. The only guarantee is that the `begin` is less than or equal to `end`.
///
/// # Properties
/// - neither `begin` nor `end` is `NaN`
/// - `begin` is less than or equal to `end`
/// - `begin` or `end` or both may be infinite so long as the previous properties hold.
#[derive(Clone, Copy, Debug)]
pub struct NonDecreasing { begin: f64, end: f64 }

impl NonDecreasing {
    /// Constructs a new instance of `NonDecreasing` beginning at `begin` and ending at `end`.
    /// This procedure will panic if either `begin` or `end` or both are `NaN`. 
    /// This procedure will panic if `begin` is greater than `end`.
    pub fn new(begin: f64, end: f64) -> Self {
        assert!(!begin.is_nan() && !end.is_nan(), 
            "Cannot construct NonDecreasing interval out of ({}, {}) because at least one argument \
             is NaN.", begin, end);
        assert!(begin <= end, "Cannot construct NonDecreasing interval out of ({}, {}) because \
            begin is greater than end.", begin, end);
        Self { begin, end }
    }

    /// Constructs a new instance of `NonDecreasing` beginning with the minimum of `a` and `b`,
    /// and ending with maximum of `a` and `b`.
    ///
    /// This procedure will panic if either `begin` or `end` or both are `NaN`.
    /// For any other set of inputs the call is guaranteed not to panic.
    ///
    /// If the ordering of `a` and `b` is already known, use [`NonDecreasing::new`] instaed
    /// to avoid the overhead of computing the minimum and maximum of the values. 
    pub fn minmax(a: f64, b: f64) -> Self {
        assert!(!a.is_nan() && !b.is_nan(), "Cannot construct NonDecreasing interval out of \
             ({}, {}) because at least one argument is NaN.", a, b);
        let begin = f64::min(a, b);
        let end = f64::max(a, b);
        Self::new(begin, end)
    }

    /// Returns the beginning of this interval (minimum). The returned value is guaranteed 
    /// not to be `NaN`.
    pub fn begin(&self) -> f64 { self.begin }

    /// Returns the end of this interval (maximum). The returned value is guaranteed not to
    ///  be `NaN`.
    pub fn end(&self) -> f64 { self.end }

    /// Reflects this interval across zero on the number line.
    pub fn reflect(&mut self) {
        let new_begin = self.end * -1.0;
        let new_end = self.begin * -1.0;
        
        self.begin = new_begin;
        self.end = new_end;
    }
}

/// A closed (includes its endpoints) non-decreasing interval of real numbers. 
/// Unlike [`NonDecreasing`], this type guarantees that its endpoints are **not** infinite in 
/// addition to them not being `NaN`.
#[derive(Clone, Copy, Debug)]
pub struct ClosedInterval { bounds: NonDecreasing }

impl From<ClosedInterval> for NonDecreasing {
    /// Discards the closed-ness of this interval and returns an ambiguous [`NonDecreasing`]
    /// interval with the same endpoints.
    fn from(value: ClosedInterval) -> Self { value.bounds }
}

impl ClosedInterval {
    /// Constructs a new instance of `ClosedInterval`.
    /// This procedure will panic if either the beginning or ending bound or both is infinite.
    pub fn new(bounds: NonDecreasing) -> Self {
        assert!(!bounds.begin().is_infinite() && !bounds.end().is_infinite(),
            "Cannot construct ClosedInterval out of ({}, {}) because at least one argument \
             is infinite.", bounds.begin(), bounds.end());
        Self { bounds }
    }

    /// Returns the minimum value within this interval. The returned value is guaranteed
    /// not to be `NaN` and not to infinite.
    pub fn begin(&self) -> f64 { self.bounds.begin() }

    /// Returns the maximum value within this interval. The returned value is guaranteed
    /// not to be `NaN` and not to be infinite.
    pub fn end(&self) -> f64 { self.bounds.end() }

    /// Computes the unsigned length of this interval. The returned value is guaranteed
    /// not to be `NaN` and not to be infinite.
    pub fn len(&self) -> f64 { self.end() - self.begin() }

    /// Determines whether `value` is an element of this closed interval. 
    ///
    /// This procedure returns true iff at least one of the following conditions are met...
    /// - `value` is equal to `begin`.
    /// - `value` is equal to `end`.
    /// - `value` is between `begin` and `end`.
    pub fn includes(&self, value: f64) -> bool { value >= self.begin() && value <= self.end() }

    /// Returns true if the interval `into_other` is equal to this interval or contained within this
    /// interval.
    pub fn covers(&self, into_other: impl Into<NonDecreasing>) -> bool {
        let other: NonDecreasing = into_other.into();
        self.includes(other.begin()) && self.includes(other.end())
    }

    /// Creates an [`OpenInterval`] whose lowerbound is equal to the beginning point of this
    /// closed interval and whose upperbound is equal to the ending point of this closed interval.
    pub fn open(&self) -> OpenInterval { OpenInterval::new(self.bounds) }
}

/// The interior of a non-decreasing interval of real numbers. An open interval **does not** contain
/// its endpoints. In contrast to [`ClosedInterval`], the upper and lowerbounds of an
/// [`OpenInterval`] may be infinite, however as with all the intervals, they may not be `NaN`.
#[derive(Clone, Copy, Debug)]
pub struct OpenInterval { pub bounds: NonDecreasing }

impl From<OpenInterval> for NonDecreasing {
    fn from(value: OpenInterval) -> Self { value.bounds }
}

impl OpenInterval {
    pub fn new(bounds: NonDecreasing) -> Self { Self { bounds } }

    pub fn lowerbound(self) -> f64 { self.bounds.begin() }
    pub fn upperbound(self) -> f64 { self.bounds.end() }

    /// Returns true if this open interval contains no points. Importantly, (infinity, infinity) 
    /// and (-infinity, -infinity) are considered empty along with every other interval of the
    /// form (x, x).
    pub fn is_empty(self) -> bool { self.lowerbound() == self.upperbound() }

    /// Determines whether this *open* interval `self` overlaps the *open* interval `other`.
    /// Open intervals are considered overlapping if there is at least one number k appearing
    /// in both intervals.
    ///
    /// If an open interval has length zero, then its lowerbound and upperbound are equal,
    /// thus it has no interior, thus it contains no numbers.
    ///
    /// ```text
    /// self :        *------>
    /// other: <------*
    /// overlaps(self, other) = false
    /// ```
    ///
    /// ```text
    /// self : <------*
    /// other:        *------>
    /// overlaps(self, other) = false
    /// ```
    ///
    /// ```text
    /// self : <------->
    /// other:     *
    /// overlaps(self, other) = false
    /// ```
    ///
    /// ```text
    /// self :     *     
    /// other: <------->
    /// overlaps(self, other) = false
    /// ```
    ///
    /// ```text
    /// self : <------>
    /// other:    *->
    /// overlaps(self, other) = true
    /// ```
    ///
    /// ```text
    /// self : <------>
    /// other:   <-*
    /// overlaps(self, other) = true
    /// ```
    ///
    /// ```text
    /// self :    *->
    /// other: <------>
    /// overlaps(self, other) = true
    /// ```
    ///
    /// ```text
    /// self :   <-*
    /// other: <------>
    /// overlaps(self, other) = true
    /// ```
    ///
    /// This function is commutative.
    pub fn overlaps(&self, other: OpenInterval) -> bool { 
        if self.is_empty() || other.is_empty() { return false; }
        return self.lowerbound() < other.upperbound()
            && self.upperbound() > other.lowerbound();
    }

    /// Determines whether this open interval `self` is disjoint with the open interval `other`.
    /// 
    /// Open intervals are considered disjoint if there is no number k appearing in both 
    /// intervals.
    ///
    /// This function is the negation of [`Self::overlaps`].
    ///
    /// This function is commutative.
    pub fn is_disjoint_with(self, other: OpenInterval) -> bool { !self.overlaps(other) }

    pub fn includes(self, value: f64) -> bool {
        self.lowerbound() < value && value < self.upperbound()
    }

    /// This is the negation of [`Self::includes`].
    pub fn excludes(self, value: f64) -> bool { !self.includes(value) }
}

// # Geometry

/// A rectangle positioned in two-dimensional space whoses sides are parallel to the axis.
/// Note the rectangale may be degenerate. In other words, either of the side lengths or
/// both may be zero.
#[derive(Clone, Copy, Debug)]
pub struct BoundingRect { pub x: ClosedInterval, pub y: ClosedInterval }

impl BoundingRect {
    pub fn area(&self) -> f64 { self.x.len() * self.y.len() }

    pub fn top_right(&self) -> Vec2D { 
        Vec2D { x: self.x.end(), y: self.y.begin() }
    }

    pub fn includes(&self, point: &Vec2D) -> bool {
        self.x.includes(point.x) && self.y.includes(point.y)
    }
}

// # Vectors

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2D { pub x: f64, pub y: f64 }

