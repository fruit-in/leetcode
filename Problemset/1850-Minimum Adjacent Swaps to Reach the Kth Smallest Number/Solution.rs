impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut num = num.into_bytes();
        let n = num.len();
        let origin = num.clone();
        let mut ret = 0;

        for _ in 0..k {
            for i in (0..n - 1).rev() {
                if num[i] >= num[i + 1] {
                    continue;
                }

                for j in (i + 1..n).rev() {
                    if num[i] >= num[j] {
                        continue;
                    }

                    num.swap(i, j);
                    for k in 0..(n - i) / 2 {
                        num.swap(i + 1 + k, n - 1 - k);
                    }
                    break;
                }
                break;
            }
        }

        for i in 0..n {
            if num[i] == origin[i] {
                continue;
            }

            for j in i + 1..n {
                if num[j] != origin[i] {
                    continue;
                }

                for k in (i + 1..=j).rev() {
                    num[k] = num[k - 1];
                }
                num[i] = origin[i];
                ret += j - i;
                break;
            }
        }

        ret as i32
    }
}
