impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut d = (arr.len() as i32 + 1) / 2;
        let mut prev = d;
        let mut ret = 0;

        for i in 0..(arr.len() / 2) {
            let j = arr.len() - 1 - i;

            ret += (arr[i] + arr[j]) * prev;
            d -= match arr.len() % 2 {
                0 => 1,
                _ => 2 * (1 - i as i32 % 2),
            };
            prev += d;
        }

        if arr.len() % 2 == 1 {
            ret += arr[arr.len() / 2] * prev;
        }

        ret
    }
}
