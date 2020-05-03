impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        arr1.iter()
            .filter(|&x| arr2.iter().all(|y| (x - y).abs() > d))
            .count() as i32
    }
}
