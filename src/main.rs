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

#[macro_export]
macro_rules! quickcheck_triangle {
    ($metric:expr, $point:ty) => {{
        fn test(a: $point, b: $point, c: $point) -> TestResult {
            (triangle_inequality($metric))(a, b, c)
        }
        quickcheck::quickcheck(test as fn($point, $point, $point) -> TestResult);
    }};
}

#[macro_export]
macro_rules! quickcheck_triangle_checked {
    ($metric:expr, $point:ty, $should_discard:expr) => {{
        fn test(a: $point, b: $point, c: $point) -> TestResult {
            (triangle_inequality_checked($metric, $should_discard))(a, b, c)
        }
        quickcheck::quickcheck(test as fn($point, $point, $point) -> TestResult);
    }};
}

fn main() {}
