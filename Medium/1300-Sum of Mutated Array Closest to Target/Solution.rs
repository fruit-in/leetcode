impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        let mut prefix_sum = 0;
        let mut diff = target;
        let mut value = 0;

        arr.push(0);
        arr.sort_unstable();

        for i in 1..arr.len() {
            let mut l = arr[i - 1];
            let mut r = arr[i];

            while l <= r {
                let m = (l + r) / 2;
                let sum = prefix_sum + m * (arr.len() - i) as i32;

                if sum == target {
                    return m;
                } else if sum < target {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }

            let mut rdiff = (prefix_sum + r * (arr.len() - i) as i32 - target).abs();
            let mut ldiff = (prefix_sum + l * (arr.len() - i) as i32 - target).abs();

            if r >= arr[i - 1] && rdiff < diff {
                diff = rdiff;
                value = r;
            }
            if l <= arr[i] && ldiff < diff {
                diff = ldiff;
                value = l;
            }

            prefix_sum += arr[i];
        }

        value
    }
}
