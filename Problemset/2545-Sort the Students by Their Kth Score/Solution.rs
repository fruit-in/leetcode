impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_unstable_by_key(|row| -row[k as usize]);

        score
    }
}
