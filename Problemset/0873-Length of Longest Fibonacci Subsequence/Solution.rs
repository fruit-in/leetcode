use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut lengths = HashMap::new();
        let last = *arr.last().unwrap();
        let mut ret = 0;

        for i in 1..arr.len() {
            for j in 0..i {
                if arr[i] - arr[j] < arr[j] {
                    let x = *lengths.get(&(arr[i] - arr[j], arr[j])).unwrap_or(&1);

                    if x > 1 {
                        lengths.insert((arr[j], arr[i]), x + 1);
                        ret = ret.max(x + 1);
                    } else if arr[i] + arr[j] <= last {
                        lengths.insert((arr[j], arr[i]), 2);
                    }
                } else if arr[i] + arr[j] <= last {
                    lengths.insert((arr[j], arr[i]), 2);
                }
            }
        }

        ret
    }
}
