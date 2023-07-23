use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1 = HashMap::new();
        for n in nums1 {
            *map1.entry(n).or_insert(0) += 1;
        }

        let mut ret = Vec::new();

        for n in nums2 {
            if let Some(x) = map1.get_mut(&n) {
                *x -= 1;
                if *x == 0 {
                    map1.remove(&n);
                } 
                ret.push(n);
            }
        }

        ret
    }
}
