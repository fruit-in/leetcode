impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;
        let mut ret = 0;

        players.sort_unstable();
        trainers.sort_unstable();

        while let Some(trainer) = trainers.pop() {
            while *players.last().unwrap_or(&0) > trainer {
                players.pop();
            }
            if players.pop().unwrap_or(i32::MAX) <= trainer {
                ret += 1;
            }
        }

        ret
    }
}
