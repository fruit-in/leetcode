impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1.len() * 6 < nums2.len() || nums2.len() * 6 < nums1.len() {
            return -1;
        }

        let mut count1 = [0; 6];
        let mut count2 = [0; 6];
        let mut diff = 0;
        let mut ret = 0;

        for &x in &nums1 {
            count1[x as usize - 1] += 1;
            diff -= x;
        }
        for &x in &nums2 {
            count2[x as usize - 1] += 1;
            diff += x;
        }

        if diff < 0 {
            diff = -diff;
            (count1, count2) = (count2, count1);
        }

        for i in 0..5 {
            count1[i] += count2[5 - i];
            if (diff + 4 - i as i32) / (5 - i as i32) <= count1[i] {
                ret += (diff + 4 - i as i32) / (5 - i as i32);
                break;
            } else {
                diff -= count1[i] * (5 - i as i32);
                ret += count1[i];
            }
        }

        ret
    }
}
