impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let mut teams = votes[0].clone().into_bytes();
        let mut counter = [[0; 26]; 26];

        for vote in votes {
            for (r, c) in vote.bytes().enumerate() {
                counter[(c - b'A') as usize][r] -= 1;
            }
        }

        teams.sort_unstable_by_key(|&c| (counter[(c - b'A') as usize], c));

        String::from_utf8(teams).unwrap()
    }
}
