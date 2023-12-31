impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let limit = (1 << requests.len()) - 1;

        for m in (1..=requests.len()).rev() {
            let mut x: i32 = (1 << m) - 1;

            while x <= limit {
                let mut change = vec![0; n as usize];

                for i in 0..requests.len() {
                    if x & (1 << i) != 0 {
                        change[requests[i][0] as usize] -= 1;
                        change[requests[i][1] as usize] += 1;
                    }
                }

                if change.iter().all(|&c| c == 0) {
                    return m as i32;
                }

                let zeros = x.trailing_zeros();
                let ones = (x >> zeros).trailing_ones();
                x = (((x >> (zeros + ones)) + 1) << (zeros + ones)) + (1 << (ones - 1)) - 1;
            }
        }

        0
    }
}
