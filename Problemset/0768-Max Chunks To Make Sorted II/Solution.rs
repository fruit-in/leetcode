use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut heap = arr.iter().collect::<BinaryHeap<_>>();
        let mut pop_count = HashMap::new();
        let mut min_num = i32::MAX;
        let mut ret = 0;

        for i in (0..arr.len()).rev() {
            *pop_count.entry(arr[i]).or_insert(0) += 1;
            min_num = min_num.min(arr[i]);

            while *pop_count.get(*heap.peek().unwrap_or(&&-1)).unwrap_or(&0) > 0 {
                *pop_count.get_mut(heap.pop().unwrap()).unwrap() -= 1;
            }

            if **heap.peek().unwrap_or(&&-1) <= min_num {
                ret += 1;
            }
        }

        ret
    }
}
