use levenshtein;

fn main() {
    fn levenshtein(a: &String, b: &String) -> usize {
        levenshtein::levenshtein(a, b)
    }

    quickcheck_triangle! {levenshtein, String};
}
