impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut l = match arr.binary_search(&x) {
            Err(i) if i == arr.len() => return arr.split_at(i - k as usize).1.to_vec(),
            Err(i) if i == 0 => return arr[0..k as usize].to_vec(),
            Err(i) if (arr[i - 1] - x).abs() <= arr[i] - x => i - 1,
            Ok(i) | Err(i) => i,
        };
        let mut r = l + 1;

        while ((r - l) as i32) < k {
            if r == arr.len() || (l > 0 && (arr[l - 1] - x).abs() <= arr[r] - x) {
                l -= 1;
            } else {
                r += 1;
            }
        }

        arr[l..r].to_vec()
    }
}
