extern crate quickcheck;
use quickcheck::{Arbitrary, TestResult};
use std::cmp::PartialOrd;
use std::ops::Add;

pub fn triangle_inequality_checked<
    Metric: 'static + Fn(&Point, &Point) -> N,
    ShouldDiscard: Fn(&Point) -> Option<TestResult>,
    Point: Arbitrary,
    N: Clone + PartialOrd + Add<Output = N>,
>(
    metric: Metric,
    should_discard: ShouldDiscard,
) -> impl Fn(Point, Point, Point) -> TestResult {
    let ret = move |a: Point, b: Point, c: Point| -> TestResult {
        match should_discard(&a).or(should_discard(&b).or(should_discard(&c))) {
            Some(result) => {
                return result;
            }
            None => (),
        };
        let side = (metric(&a, &b), metric(&b, &c), metric(&c, &a));
        let result = side.0 <= side.1.clone() + side.2.clone()
            && side.1 <= side.2.clone() + side.0.clone()
            && side.2 <= side.0 + side.1;
        TestResult::from_bool(result)
    };
    ret
}

pub fn triangle_inequality<
    Metric: 'static + Fn(&Point, &Point) -> N,
    Point: Arbitrary,
    N: Clone + PartialOrd + Add<Output = N>,
>(
    metric: Metric,
) -> impl Fn(Point, Point, Point) -> TestResult {
    triangle_inequality_checked(metric, |_| None)
}

macro_rules! quickcheck_triangle {
    ($metric:expr, $point:ty) => {{
        fn test(a: $point, b: $point, c: $point) -> TestResult {
            (triangle_inequality($metric))(a, b, c)
        }
        quickcheck::quickcheck(test as fn($point, $point, $point) -> TestResult);
    }};
}

macro_rules! quickcheck_triangle_checked {
    ($metric:expr, $point:ty, $should_discard:expr) => {{
        fn test(a: $point, b: $point, c: $point) -> TestResult {
            (triangle_inequality_checked($metric, $should_discard))(a, b, c)
        }
        quickcheck::quickcheck(test as fn($point, $point, $point) -> TestResult);
    }};
}

type Point2D = (f64, f64);

fn pythagorean(a: &Point2D, b: &Point2D) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn main() {
    ///---------------------------------------------------
    /// This will quickly result in "TEST FAILED. Arguments: ((0.0, 0.0), (NaN, 0.0), (0.0, 0.0))"
    //quickcheck_triangle! {pythagorean, Point2D};

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
