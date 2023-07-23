impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let len = arr.len() * 9 / 10;

        arr.sort_unstable();

        arr.iter().skip(len / 18).take(len).sum::<i32>() as f64 / (len as f64)
    }
}
