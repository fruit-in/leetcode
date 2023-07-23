impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let mut total_sum: i32 = a.iter().sum();
        if total_sum % 3 != 0 {
            return false;
        }

        let mut i = 0;
        let mut part_sum = 0;
        while i < a.len() {
            part_sum += a[i];
            if part_sum == total_sum / 3 {
                break;
            }
            i += 1;
        }

        let mut j = a.len() - 1;
        part_sum = 0;
        while j > 0 {
            part_sum += a[j];
            if part_sum == total_sum / 3 {
                break;
            }
            j -= 1;
        }

        i + 1 < j
    }
}
