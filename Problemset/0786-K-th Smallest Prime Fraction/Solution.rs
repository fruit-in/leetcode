use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Fraction(i32, i32);

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.0 as i64 * other.1 as i64;
        let y = self.1 as i64 * other.0 as i64;

        y.cmp(&x)
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();

        for i in 0..arr.len() - 1 {
            heap.push((Fraction(arr[i], arr[arr.len() - 1]), i, arr.len() - 1));
        }

        for _ in 0..k - 1 {
            let (_, i, j) = heap.pop().unwrap();

            if i < j - 1 {
                heap.push((Fraction(arr[i], arr[j - 1]), i, j - 1));
            }
        }

        let (_, i, j) = heap.pop().unwrap();

        vec![arr[i], arr[j]]
    }
}
