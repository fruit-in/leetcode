use std::collections::HashMap;

struct FindSumPairs {
    nums2: Vec<i32>,
    count1: HashMap<i32, i32>,
    count2: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut count1 = HashMap::new();
        let mut count2 = HashMap::new();

        for &num in &nums1 {
            *count1.entry(num).or_insert(0) += 1;
        }
        for &num in &nums2 {
            *count2.entry(num).or_insert(0) += 1;
        }

        Self {
            nums2,
            count1,
            count2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        *self.count2.get_mut(&self.nums2[index as usize]).unwrap() -= 1;
        self.nums2[index as usize] += val;
        *self.count2.entry(self.nums2[index as usize]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.count1
            .iter()
            .map(|(&k, &v)| v * *self.count2.get(&(tot - k)).unwrap_or(&0))
            .sum()
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
