impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while numbers[i] + numbers[j] != target {
            if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        vec![i as i32 + 1, j as i32 + 1]
    }
}
