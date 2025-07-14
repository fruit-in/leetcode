impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut new_ranges = vec![];
        let mut i = 0;
        let mut ret = 0;

        for (j, &r) in ranges.iter().enumerate() {
            let (start1, end1) = (j as i32 - r, j as i32 + r);
            let mut flag = true;

            while let Some(&(start2, end2)) = new_ranges.last() {
                if start1 <= start2 {
                    new_ranges.pop();
                    continue;
                }
                flag = end2 < end1;
                break;
            }

            if flag {
                new_ranges.push((start1, end1));
            }
        }

        for j in 0..new_ranges.len() {
            if i >= n {
                break;
            } else if new_ranges[j].0 > i {
                return -1;
            } else if j == new_ranges.len() - 1 || new_ranges[j + 1].0 > i {
                i = new_ranges[j].1;
                ret += 1;
            }
        }

        if i < n {
            return -1;
        }

        ret
    }
}
