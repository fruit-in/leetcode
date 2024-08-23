impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut sum = 0;
        let mut pairs = vec![];
        let mut ret = -1;

        for j in 0..arr.len() {
            sum += arr[j];
            while i <= j && sum > target {
                sum -= arr[i];
                i += 1;
            }

            if sum == target {
                match pairs.binary_search(&(i as i32, -1)) {
                    Err(0) | Ok(_) => (),
                    Err(k) => {
                        let x = pairs[k - 1].0 - pairs[k - 1].1 + (j - i) as i32 + 2;
                        if ret == -1 || ret > x {
                            ret = x;
                        }
                    }
                }

                let (a, b) = *pairs.last().unwrap_or(&(i32::MAX, 0));
                if ((j - i) as i32) < a - b {
                    pairs.push((j as i32, i as i32));
                }
            }
        }

        ret
    }
}
