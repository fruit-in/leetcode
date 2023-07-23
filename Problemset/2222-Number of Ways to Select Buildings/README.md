# 2222. Number of Ways to Select Buildings
You are given a **0-indexed** binary string `s` which represents the types of buildings along a street where:

* `s[i] = '0'` denotes that the <code>i<sup>th</sup></code> building is an office and
* `s[i] = '1'` denotes that the <code>i<sup>th</sup></code> building is a restaurant.

As a city official, you would like to **select** 3 buildings for random inspection. However, to ensure variety, **no two consecutive** buildings out of the **selected** buildings can be of the same type.

* For example, given `s = "001101"`, we cannot select the <code>1<sup>st</sup></code>, <code>3<sup>rd</sup></code>, and <code>5<sup>th</sup></code> buildings as that would form `"011"` which is **not** allowed due to having two consecutive buildings of the same type.

Return *the **number of valid ways** to select 3 buildings*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "001101"
<strong>Output:</strong> 6
<strong>Explanation:</strong>
The following sets of indices selected are valid:
- [0,2,4] from "001101" forms "010"
- [0,3,4] from "001101" forms "010"
- [1,2,4] from "001101" forms "010"
- [1,3,4] from "001101" forms "010"
- [2,4,5] from "001101" forms "101"
- [3,4,5] from "001101" forms "101"
No other selection is valid. Thus, there are 6 total ways.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "11100"
<strong>Output:</strong> 0
<strong>Explanation:</strong> It can be shown that there are no valid selections.
</pre>

#### Constraints:
* <code>3 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut count0 = vec![0; s.len()];
        let mut count1 = vec![0; s.len()];

        for (i, building) in s.chars().enumerate() {
            if i > 0 {
                count0[i] = count0[i - 1];
                count1[i] = count1[i - 1];
            }

            count0[i] += (building == '0') as i64;
            count1[i] += (building == '1') as i64;
        }

        s.chars()
            .enumerate()
            .map(|(i, building)| match building {
                '0' => count1[i] * (count1[s.len() - 1] - count1[i]),
                _ => count0[i] * (count0[s.len() - 1] - count0[i]),
            })
            .sum()
    }
}
```
