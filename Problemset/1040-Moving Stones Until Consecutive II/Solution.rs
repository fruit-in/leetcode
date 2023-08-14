impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let n = stones.len();
        let mut stones = stones;
        let mut i = 0;
        let mut answer = vec![i32::MAX, 0];

        stones.sort_unstable();

        for j in 0..n {
            while i < n && stones[i] <= stones[j] + n as i32 - 1 {
                i += 1;
            }
            answer[0] = answer[0].min((j + n - i) as i32);
        }
        if answer[0] == 1
            && ((stones[n - 2] - stones[0] + 2 == n as i32 && stones[n - 1] - stones[n - 2] > 2)
                || (stones[n - 1] - stones[1] + 2 == n as i32 && stones[1] - stones[0] > 2))
        {
            answer[0] = 2;
        }

        answer[1] = stones[n - 1] - stones[0] - n as i32 + 2
            - (stones[1] - stones[0]).min(stones[n - 1] - stones[n - 2]);

        answer
    }
}
