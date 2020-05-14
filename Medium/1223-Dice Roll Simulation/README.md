# 1223. Dice Roll Simulation
A die simulator generates a random number from 1 to 6 for each roll. You introduced a constraint to the generator such that it cannot roll the number ```i``` more than ```rollMax[i]``` (1-indexed) **consecutive** times.

Given an array of integers ```rollMax``` and an integer ```n```, return the number of distinct sequences that can be obtained with exact ```n``` rolls.

Two sequences are considered different if at least one element differs from each other. Since the answer may be too large, return it modulo ```10^9 + 7```.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2, rollMax = [1,1,2,2,2,3]
<strong>Output:</strong> 34
<strong>Explanation:</strong> There will be 2 rolls of die, if there are no constraints on the die, there are 6 * 6 = 36 possible combinations. In this case, looking at rollMax array, the numbers 1 and 2 appear at most once consecutively, therefore sequences (1,1) and (2,2) cannot occur, so the final answer is 36-2 = 34.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, rollMax = [1,1,1,1,1,1]
<strong>Output:</strong> 30
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3, rollMax = [1,1,1,2,2,3]
<strong>Output:</strong> 181
</pre>

#### Constraints:
* ```1 <= n <= 5000```
* ```rollMax.length == 6```
* ```1 <= rollMax[i] <= 15```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; 16]; 7]; n as usize];
        (1..=6).for_each(|j| dp[0][j][0] = 1);
        let mut ret = 0;

        for i in 1..(n as usize) {
            for j in 1..=6 {
                let f = |x: usize| {
                    dp[i - 1][x][..(roll_max[x - 1] as usize)]
                        .iter()
                        .fold(0, |acc, x| (acc + x) % M)
                };
                dp[i][j][0] = (1..=6)
                    .filter(|&x| x != j)
                    .map(|x| f(x))
                    .fold(0, |acc, x| (acc + x) % M);
                for k in 1..(roll_max[j - 1] as usize).min(i + 1) {
                    dp[i][j][k] = dp[i - 1][j][k - 1];
                }
            }
        }

        for j in 1..=6 {
            for k in 0..(roll_max[j - 1].min(n) as usize) {
                ret = (ret + dp[n as usize - 1][j][k]) % M;
            }
        }

        ret
    }
}
```
