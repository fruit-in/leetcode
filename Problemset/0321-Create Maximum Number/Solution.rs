impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let m = nums1.len();
        let n = nums2.len();
        let mut ret = vec![0; k];

        for k1 in 0.max(k.saturating_sub(n))..=m.min(k) {
            let k2 = k - k1;
            let mut stack1 = vec![];
            let mut stack2 = vec![];

            for i in 0..m {
                while m - i + stack1.len() > k1 && nums1[i] > *stack1.last().unwrap_or(&9) {
                    stack1.pop();
                }
                stack1.push(nums1[i]);
            }
            while stack1.len() > k1 {
                stack1.pop();
            }
            for i in 0..n {
                while n - i + stack2.len() > k2 && nums2[i] > *stack2.last().unwrap_or(&9) {
                    stack2.pop();
                }
                stack2.push(nums2[i]);
            }
            while stack2.len() > k2 {
                stack2.pop();
            }

            let mut i = 0;
            let mut j = 0;
            let mut tmp = vec![];
            let mut flag = false;

            stack1.push(-1);
            stack2.push(-2);

            while i < k1 || j < k2 {
                if j == k2 || stack1[i] > stack2[j] {
                    tmp.push(stack1[i]);
                    i += 1;
                } else if i == k1 || stack1[i] < stack2[j] {
                    tmp.push(stack2[j]);
                    j += 1;
                } else {
                    for l in 1..=(k1 - i).min(k2 - j) {
                        if stack1[i + l] > stack2[j + l] {
                            tmp.push(stack1[i]);
                            i += 1;
                            break;
                        } else if stack1[i + l] < stack2[j + l] {
                            tmp.push(stack2[j]);
                            j += 1;
                            break;
                        }
                    }
                }

                if tmp[i + j - 1] > ret[i + j - 1] {
                    flag = true;
                } else if !flag && tmp[i + j - 1] < ret[i + j - 1] {
                    break;
                }
            }

            if flag {
                ret = tmp;
            }
        }

        ret
    }
}
