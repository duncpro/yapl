use crate::math::{ClosedInterval, NonDecreasing, OpenInterval};
use crate::misc::{SegVec, Push};

#[derive(Clone, Copy)]
pub struct PlotFnParams {
    /// The interval of the domain to plot.
    pub domain: ClosedInterval,

    /// The interval of the codomain to plot.
    pub codomain: ClosedInterval,

    /// The minimum number of times the domain shall be bisected. The minimnum number of linear 
    /// interpolants in the resultant graph is 2^min_depth.
    /// 
    /// # Picking the Value
    ///
    /// The caller must pick this value using their prior knowledge of the shape of the function's 
    /// graph in the given domain. 
    pub min_depth: usize,

    /// The greatest tolerable absolute difference between the function's approximated value
    /// and the function's actual value at the midpoint of the domain interval. 
    /// 
    /// This threshold is used to halt the bisection of an interval prior to achieving `max_depth`
    /// but only after `min_depth` has been achieved.
    ///
    /// # Relationship to `min_depth`
    /// 
    /// To understand what `error_tolerance` does, it helps to be familiar with the `min_depth` 
    /// parameter first. It is important that the caller choses `min_depth` carefully before
    /// adjusting the `error_tolerance`. For example, consider the following graph....
    ///
    /// ```text
    ///      *       *
    ///     / \     / \ 
    /// ___/   \___/   \___
    ///
    ///  |-------|-------|
    /// ```
    ///
    /// If the caller fails to pick a `min_depth` greater than 0, this graph will be rendered
    /// as a straight line, even if the `error_tolerance` is decreased all the way to 0! 
    ///
    /// This happens because the value of the midpoint of the linear interpolant is equal to the
    /// actual value of the function indicating that the linear interpolant is a "good"
    /// approximation of the behavior of the function over that interval of the domain. But the
    /// interval is so large it could never be accurately represented by so few linear interpolants
    /// (in this extreme case, a single one).
    ///
    /// The point being, decreasting `error_tolerance` is useful for smoothing an alraedy 
    /// approximately accurate graph but not for capture macroscopic changes in the function's
    /// beheavior.
    ///
    /// # Picking the Value 
    ///
    /// This value should be equal to the minimum visible distance on the display surface in terms
    /// of the length of the codomain. 
    ///
    /// For example, given the codomain [-k, k] and the window height h pixels, the `error_tolerance`
    /// should be set to 2k / h.
    pub error_tolerance: f64,

    /// The minimum distance along the x-axis that is visible given the scale of the graph
    /// and the resolution of the display.
    ///
    /// Once the domain of a bisection has length less than `zero_tolerance` it is pruned
    /// from the graph entirely. There will be no stroke over this interval.
    ///
    /// This number is particularly important when graphing periodic functions such as sin(1/x).
    /// This is because sin(1/x) has an infinite number of periods within a finite interval.
    /// There is no number of bisections that will ever make the linear interpolants satisfy the
    /// error threshold. So another halting condition must be introduced. 
    ///
    /// # Picking the Value
    ///
    /// This value should be below the minimum visible distance on the display surface in
    /// terms of the length of the domain.
    ///
    /// For example, given the domain [-k, k] and a window width h pixels, the `zero_tolernace`
    /// should be below 2k / h.
    ///
    /// Exactly how small this value is will depend on the function being plotted. 
    pub zero_tolerance: f64
}


#[derive(Clone, Copy, PartialEq)]
pub struct Anchor { pub input: f64 }

#[derive(Clone, Copy, PartialEq)]
pub enum Node { Break, Anchor(Anchor) }

#[derive(Default, Debug)]
pub struct Stats {
    pub accept: usize,
    pub prune_outside_viewport_finite: usize,
    pub prune_outside_viewport_infinite: usize,
    pub prune_zero_tolerance: usize,
    pub breaks: usize,
    pub duration: std::time::Duration
}

#[derive(Clone, Copy)]
struct State { domain: ClosedInterval, depth: usize }

// # Plotting Algorithm

pub fn plotfn(f: &Box<dyn Fn(f64) -> f64>, nodes: &mut SegVec<Node>, params: PlotFnParams) -> Stats
{
    assert!(params.error_tolerance >= 0.0);
    assert!(params.zero_tolerance >= 0.0);
    return bisect(f, params, nodes);
}

fn bisect(f: &Box<dyn Fn(f64) -> f64>, params: PlotFnParams, nodes: &mut SegVec<Node>) -> Stats {
    let mut stack: Vec<State> = vec![State { domain: params.domain, depth: 0 }];
    let mut stats = Stats::default();
    let begin = std::time::Instant::now();

    while let Some(state) = stack.pop() {
        // The point in the domain to split this interpolant.
        let splitpoint = (state.domain.begin() + state.domain.end()) / 2.0;

        if state.depth >= params.min_depth {
            // The exact value of *f* at the left side (more negative) of the domain.
            let left_y = f(state.domain.begin()); // Might be NaN!
            // The exact value of *f* at the right side (more positive) of the domain.
            let right_y = f(state.domain.end());  // Might be NaN!
            
            // The interpolant is only defined if its endpoints are defined.
            if !left_y.is_nan() && !right_y.is_nan() {
                let interpolant_y_interval = OpenInterval::new(NonDecreasing::minmax(left_y, right_y));
                
                if params.codomain.open().is_disjoint_with(interpolant_y_interval) {
                    stats.prune_outside_viewport_finite += 1;
                    continue; 
                }
                  
                let approximate_value = (left_y + right_y) / 2.0; 
                let actual_value = f(splitpoint); // Might be NaN!
                    
                if !actual_value.is_nan() && !approximate_value.is_nan() {
                    let error = (approximate_value - actual_value).abs();  
                    let is_within_error_tolerance = error <= params.error_tolerance;
                    let is_framed = params.codomain.covers(interpolant_y_interval);
                    // If further bisection will make the segment invisible on the display
                    // surface, stop bisecting and just accept the approximation, no matter
                    // how innacurate it is. At this microscopic scale, the innacuracy will
                    // likely be negligible.
                    let will_disappear = state.domain.len() <= 2.0 * params.zero_tolerance;
                    if (is_within_error_tolerance || will_disappear) && is_framed {
                        let left_anchor = Node::Anchor(Anchor { input: state.domain.begin() });
                        if nodes.as_slice().last().copied() != Some(left_anchor) {
                            nodes.push(Node::Break);
                            nodes.push(left_anchor);
                            stats.breaks += 1;
                        }
                        nodes.push(Node::Anchor(Anchor { input: state.domain.end() }));
                        stats.accept += 1;
                        continue;
                    }
                }
            }
            
            if left_y.is_infinite() && right_y.is_finite() {
                if params.codomain.open().excludes(right_y) {
                    stats.prune_outside_viewport_infinite += 1;
                    continue;
                }
            }

            if left_y.is_finite() && right_y.is_infinite() {
                if params.codomain.open().excludes(left_y) {
                    stats.prune_outside_viewport_infinite += 1;
                    continue;
                }
            }
        }
 
        if state.domain.len() < params.zero_tolerance { 
            stats.prune_zero_tolerance += 1;
            continue 
        };

        stack.push(State { 
            domain: ClosedInterval::new(NonDecreasing::new(splitpoint, state.domain.end())), 
            depth: state.depth + 1  
        });
        stack.push(State { 
            domain: ClosedInterval::new(NonDecreasing::new(state.domain.begin(), splitpoint)), 
            depth: state.depth + 1
        });
    }

    stats.duration = begin.elapsed();
    return stats;
}

