impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut max = [i32::MIN; 4];
        let mut min = [i32::MAX; 4];

        for i in 0..arr1.len() {
            max[0] = max[0].max(i as i32 + arr1[i] + arr2[i]);
            max[1] = max[1].max(i as i32 + arr1[i] - arr2[i]);
            max[2] = max[2].max(i as i32 - arr1[i] + arr2[i]);
            max[3] = max[3].max(i as i32 - arr1[i] - arr2[i]);
            min[0] = min[0].min(i as i32 + arr1[i] + arr2[i]);
            min[1] = min[1].min(i as i32 + arr1[i] - arr2[i]);
            min[2] = min[2].min(i as i32 - arr1[i] + arr2[i]);
            min[3] = min[3].min(i as i32 - arr1[i] - arr2[i]);
        }

        (0..4).map(|i| max[i] - min[i]).max().unwrap()
    }
}
