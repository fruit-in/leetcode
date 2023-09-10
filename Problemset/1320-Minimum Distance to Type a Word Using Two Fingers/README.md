# 1320. Minimum Distance to Type a Word Using Two Fingers
![](https://assets.leetcode.com/uploads/2020/01/02/leetcode_keyboard.png)

You have a keyboard layout as shown above in the **X-Y** plane, where each English uppercase letter is located at some coordinate.

* For example, the letter `'A'` is located at coordinate `(0, 0)`, the letter `'B'` is located at coordinate `(0, 1)`, the letter `'P'` is located at coordinate `(2, 3)` and the letter `'Z'` is located at coordinate `(4, 1)`.

Given the string `word`, return *the minimum total **distance** to type such string using only two fingers*.

The **distance** between coordinates <code>(x<sub>1</sub>, y<sub>1</sub>)</code> and <code>(x<sub>2</sub>, y<sub>2</sub>)</code> is <code>|x<sub>1</sub> - x<sub>2</sub>| + |y<sub>1</sub> - y<sub>2</sub>|</code>.

**Note** that the initial positions of your two fingers are considered free so do not count towards your total distance, also your two fingers do not have to start at the first letter or the first two letters.

#### Example 1:
<pre>
<strong>Input:</strong> word = "CAKE"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Using two fingers, one optimal way to type "CAKE" is:
Finger 1 on letter 'C' -> cost = 0
Finger 1 on letter 'A' -> cost = Distance from letter 'C' to letter 'A' = 2
Finger 2 on letter 'K' -> cost = 0
Finger 2 on letter 'E' -> cost = Distance from letter 'K' to letter 'E' = 1
Total distance = 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "HAPPY"
<strong>Output:</strong> 6
<strong>Explanation:</strong> Using two fingers, one optimal way to type "HAPPY" is:
Finger 1 on letter 'H' -> cost = 0
Finger 1 on letter 'A' -> cost = Distance from letter 'H' to letter 'A' = 2
Finger 2 on letter 'P' -> cost = 0
Finger 2 on letter 'P' -> cost = Distance from letter 'P' to letter 'P' = 0
Finger 1 on letter 'Y' -> cost = Distance from letter 'A' to letter 'Y' = 4
Total distance = 6
</pre>

#### Constraints:
* `2 <= word.length <= 300`
* `word` consists of uppercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word = word
            .bytes()
            .map(|b| (b - b'A') as usize)
            .collect::<Vec<_>>();
        let mut xy = (0..26).map(|i: i32| (i / 6, i % 6)).collect::<Vec<_>>();
        let mut dp = vec![vec![vec![i32::MAX; 26]; 26]; word.len() + 1];
        let mut ret = i32::MAX;
        dp[0] = vec![vec![0; 26]; 26];

        for i in 0..word.len() {
            let (x1, y1) = xy[word[i]];

            for j in 0..26 {
                let (x2, y2) = xy[j];

                for k in 0..26 {
                    if dp[i][j][k] == i32::MAX {
                        continue;
                    }

                    let (x3, y3) = xy[k];

                    dp[i + 1][word[i]][k] =
                        dp[i + 1][word[i]][k].min(dp[i][j][k] + (x1 - x2).abs() + (y1 - y2).abs());
                    dp[i + 1][j][word[i]] =
                        dp[i + 1][j][word[i]].min(dp[i][j][k] + (x1 - x3).abs() + (y1 - y3).abs());
                }
            }
        }

        for i in 0..26 {
            ret = ret.min(dp[word.len()][*word.last().unwrap()][i]);
            ret = ret.min(dp[word.len()][i][*word.last().unwrap()]);
        }

        ret
    }
}
```
