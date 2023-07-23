use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut ret = vec![];

        for bar in barcodes {
            *count.entry(bar).or_insert(0) += 1;
        }
        for (k, v) in count.into_iter() {
            heap.push((v, k));
        }

        while let Some(mut max0) = heap.pop() {
            ret.push(max0.1);
            max0.0 -= 1;
            if let Some(mut max1) = heap.pop() {
                ret.push(max1.1);
                max1.0 -= 1;
                if max1.0 > 0 {
                    heap.push(max1);
                }
            }
            if max0.0 > 0 {
                heap.push(max0);
            }
        }

        ret
    }
}
