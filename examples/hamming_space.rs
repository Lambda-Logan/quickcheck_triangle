use space::{Hamming, MetricPoint};

fn main() {
    fn hamming<T: Copy>(a: &T, b: &T) -> u64
    where
        Hamming<T>: MetricPoint,
    {
        Hamming(*a).distance(&Hamming(*b))
    }

    quickcheck_triangle!(hamming, u8);
    quickcheck_triangle!(hamming, u32);
    quickcheck_triangle!(hamming, u64);
    quickcheck_triangle!(hamming, u128);
}
