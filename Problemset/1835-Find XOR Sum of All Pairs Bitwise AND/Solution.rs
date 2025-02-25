impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let xor1 = arr1.iter().fold(0, |acc, x| acc ^ x);
        arr2.iter().fold(0, |acc, x| acc ^ (x & xor1))
    }
}
