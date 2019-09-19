# 1128. Number of Equivalent Domino Pairs
Given a list of ```dominoes```, ```dominoes[i] = [a, b]``` is *equivalent* to ```dominoes[j] = [c, d]``` if and only if either (```a==c``` and ```b==d```), or (```a==d``` and ```b==c```) - that is, one domino can be rotated to be equal to another domino.

Return the number of pairs ```(i, j)``` for which ```0 <= i < j < dominoes.length```, and ```dominoes[i]``` is equivalent to ```dominoes[j]```.

#### Example 1:
<pre>
<strong>Input:</strong> dominoes = [[1,2],[2,1],[3,4],[5,6]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* ```1 <= dominoes.length <= 40000```
* ```1 <= dominoes[i][j] <= 9```

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut cnt = [0; 100];
        let mut ret = 0;
        for domino in dominoes {
            let tens = domino[0].min(domino[1]) as usize;
            let units = domino[0].max(domino[1]) as usize;
            ret += cnt[tens * 10 + units];
            cnt[tens * 10 + units] += 1;
        }
        ret
    }
}
```
