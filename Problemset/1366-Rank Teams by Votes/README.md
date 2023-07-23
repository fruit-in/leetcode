# 1366. Rank Teams by Votes
In a special ranking system, each voter gives a rank from highest to lowest to all teams participated in the competition.

The ordering of teams is decided by who received the most position-one votes. If two or more teams tie in the first position, we consider the second position to resolve the conflict, if they tie again, we continue this process until the ties are resolved. If two or more teams are still tied after considering all positions, we rank them alphabetically based on their team letter.

Given an array of strings `votes` which is the votes of all voters in the ranking systems. Sort all teams according to the ranking system described above.

Return *a string of all teams* **sorted** by the ranking system.

#### Example 1:
<pre>
<strong>Input:</strong> votes = ["ABC","ACB","ABC","ACB","ACB"]
<strong>Output:</strong> "ACB"
<strong>Explanation:</strong> Team A was ranked first place by 5 voters. No other team was voted as first place so team A is the first team.
Team B was ranked second by 2 voters and was ranked third by 3 voters.
Team C was ranked second by 3 voters and was ranked third by 2 voters.
As most of the voters ranked C second, team C is the second team and team B is the third.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> votes = ["WXYZ","XYZW"]
<strong>Output:</strong> "XWYZ"
<strong>Explanation:</strong> X is the winner due to tie-breaking rule. X has same votes as W for the first position but X has one vote as second position while W doesn't have any votes as second position.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> votes = ["ZMNAGUEDSJYLBOPHRQICWFXTVK"]
<strong>Output:</strong> "ZMNAGUEDSJYLBOPHRQICWFXTVK"
<strong>Explanation:</strong> Only one voter so his votes are used for the ranking.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> votes = ["BCA","CAB","CBA","ABC","ACB","BAC"]
<strong>Output:</strong> "ABC"
<strong>Explanation:</strong>
Team A was ranked first by 2 voters, second by 2 voters and third by 2 voters.
Team B was ranked first by 2 voters, second by 2 voters and third by 2 voters.
Team C was ranked first by 2 voters, second by 2 voters and third by 2 voters.
There is a tie and we rank teams ascending by their IDs.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> votes = ["M","M","M","M"]
<strong>Output:</strong> "M"
<strong>Explanation:</strong> Only team M in the competition so it has the first rank.
</pre>

#### Constraints:
* `1 <= votes.length <= 1000`
* `1 <= votes[i].length <= 26`
* `votes[i].length == votes[j].length` for `0 <= i, j < votes.length`.
* `votes[i][j]` is an English **upper-case** letter.
* All characters of `votes[i]` are unique.
* All the characters that occur in `votes[0]` **also occur** in `votes[j]` where `1 <= j < votes.length`.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
