impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m as i64 * k as i64 > bloom_day.len() as i64 {
            return -1;
        }

        let mut low = *bloom_day.iter().min().unwrap();
        let mut high = *bloom_day.iter().max().unwrap();

        while low < high {
            let day = (low + high) / 2;
            let mut bouquets = 0;
            let mut i = 0;
            let mut j = 0;

            while j < bloom_day.len() {
                if bloom_day[i] > day && bloom_day[j] <= day {
                    i = j;
                }

                if bloom_day[j] <= day && j - i + 1 == k as usize {
                    bouquets += 1;
                    i = j + 1;
                } else if bloom_day[j] > day {
                    i = j + 1;
                }

                j += 1;
            }

            if bouquets < m {
                low = day + 1;
            } else {
                high = day;
            }
        }

        high
    }
}
