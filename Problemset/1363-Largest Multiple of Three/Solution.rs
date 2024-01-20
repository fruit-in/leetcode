impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut count = [0; 10];
        let mut sum_rem = 0;
        let mut min_rem = [vec![], vec![], vec![]];
        let mut ret = vec![];

        for d in digits {
            count[d as usize] += 1;
            sum_rem = (sum_rem + d) % 3;
        }

        for d in 1..9 {
            if min_rem[d % 3].len() < 2 && count[d] > 0 {
                min_rem[d % 3].push(d);
                if min_rem[d % 3].len() < 2 && count[d] > 1 {
                    min_rem[d % 3].push(d);
                }
            }
        }

        if sum_rem > 0 {
            if !min_rem[sum_rem as usize].is_empty() {
                count[min_rem[sum_rem as usize][0]] -= 1;
            } else {
                count[min_rem[3 - sum_rem as usize][0]] -= 1;
                count[min_rem[3 - sum_rem as usize][1]] -= 1;
            }
        }

        for d in (0..10).rev() {
            while count[d] > 0 {
                count[d] -= 1;
                ret.push(d as u8 + b'0');
            }
        }

        if *ret.get(0).unwrap_or(&1) == b'0' {
            return 0.to_string();
        }

        String::from_utf8(ret).unwrap()
    }
}
