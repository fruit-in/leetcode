impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut zeroes = 0;

        for i in 0..arr.len() {
            if i + zeroes / 2 >= arr.len() - 1 {
                if arr[i] == 0 && i + zeroes / 2 == arr.len() - 1 {
                    zeroes += 1;
                }
                break;
            }

            if arr[i] == 0 {
                zeroes += 2;
            }
        }

        for i in (0..arr.len()).rev() {
            arr[i] = arr[i - zeroes / 2];
            if arr[i] == 0 {
                zeroes -= 1;
            }
        }
    }
}
