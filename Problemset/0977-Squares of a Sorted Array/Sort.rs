impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut squares: Vec<i32> = a.iter().map(|&x| x * x).collect();
        squares.sort_unstable();
        squares
    }
}
