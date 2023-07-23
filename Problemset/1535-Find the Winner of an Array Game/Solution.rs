impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut wins = 0;

        for &x in &arr[1..] {
            if x > winner {
                winner = x;
                wins = 0;
            }
            wins += 1;
            if wins == k {
                break;
            }
        }

        winner
    }
}
