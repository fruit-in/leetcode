impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        for i in 0..len {
            let tg = target - numbers[i];
            let mut left = i + 1;
            let mut right = len - 1;
            let mut mid = (left + right) / 2;
            while left <= right {
                if numbers[mid] == tg {
                    return vec![i as i32 + 1, mid as i32 + 1];
                } else if numbers[mid] < tg {
                    left = mid + 1;
                    mid = (left + right) / 2;
                } else {
                    right = mid - 1;
                    mid = (left + right) / 2;
                }
            }
        }
        Vec::new()
    }
}
