# 354. Russian Doll Envelopes
You are given a 2D array of integers `envelopes` where <code>envelopes[i] = [w<sub>i</sub>, h<sub>i</sub>]</code> represents the width and the height of an envelope.

One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.

Return *the maximum number of envelopes you can Russian doll (i.e., put one inside the other)*.

**Note:** You cannot rotate an envelope.

#### Example 1:
<pre>
<strong>Input:</strong> envelopes = [[5,4],[6,4],[6,7],[2,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> envelopes = [[1,1],[1,1],[1,1]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>1 <= envelopes.length <= 10<sup>5</sup></code>
* `envelopes[i].length == 2`
* <code>1 <= w<sub>i</sub>, h<sub>i</sub> <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let mut lis = vec![];

        envelopes.sort_unstable_by_key(|e| (e[0], -e[1]));

        for i in 0..envelopes.len() {
            if let Err(j) = lis.binary_search(&envelopes[i][1]) {
                if j == lis.len() {
                    lis.push(envelopes[i][1]);
                } else {
                    lis[j] = envelopes[i][1];
                }
            }
        }

        lis.len() as i32
    }
}
```
