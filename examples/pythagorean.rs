type Point2D = (f64, f64);

fn pythagorean(a: &Point2D, b: &Point2D) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn main() {
    ///---------------------------------------------------
    /// This will quickly result in "TEST FAILED. Arguments: ((0.0, 0.0), (NaN, 0.0), (0.0, 0.0))"
    /// If this is not the behavoir you want, use the quickcheck_triangle_checked! macro below, which has 1 extra argument
    quickcheck_triangle! {pythagorean, Point2D};

    ///---------------------------------------------------
    ///To discard certain input, we have an extra argument that's a Fn(&Point)->Option<quickcheck::TestResult>
    ///Simply have the function return Some(TestResult::discard())
    /// This will pass
    quickcheck_triangle_checked! {
        pythagorean,
        Point2D,
        |point: &Point2D| -> Option<TestResult> {
            // lets say we want to ignore nan or inf values in our points
            if !point.0.is_finite() || !point.1.is_finite() {
                return Some(TestResult::discard());
            }
            None
        }
    }
}
