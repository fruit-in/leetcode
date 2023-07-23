# 1337. The K Weakest Rows in a Matrix
Given a ```m * n``` matrix ```mat``` of *ones* (representing soldiers) and *zeros* (representing civilians), return the indexes of the ```k``` weakest rows in the matrix ordered from the weakest to the strongest.

A row ***i*** is weaker than row ***j***, if the number of soldiers in row ***i*** is less than the number of soldiers in row ***j***, or they have the same number of soldiers but ***i*** is less than ***j***. Soldiers are **always** stand in the frontier of a row, that is, always *ones* may appear first and then *zeros*.

#### Example 1:
<pre>
<strong>Input:</strong> mat =
[[1,1,0,0,0],
 [1,1,1,1,0],
 [1,0,0,0,0],
 [1,1,0,0,0],
 [1,1,1,1,1]],
k = 3
<strong>Output:</strong> [2,0,3]
<strong>Explanation:</strong>
The number of soldiers for each row is:
row 0 -> 2
row 1 -> 4
row 2 -> 1
row 3 -> 2
row 4 -> 5
Rows ordered from the weakest to the strongest are [2,0,3,1,4]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> mat =
[[1,0,0,0],
 [1,1,1,1],
 [1,0,0,0],
 [1,0,0,0]],
k = 2
<strong>Output:</strong> [0,2]
<strong>Explanation:</strong>
The number of soldiers for each row is:
row 0 -> 1
row 1 -> 4
row 2 -> 1
row 3 -> 1
Rows ordered from the weakest to the strongest are [0,2,3,1]
</pre>

#### Constraints:
* ```m == mat.length```
* ```n == mat[i].length```
* ```2 <= n, m <= 100```
* ```1 <= k <= m```
* ```matrix[i][j]``` is either 0 **or** 1.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mat = mat.iter().map(|r| r.iter().sum()).collect::<Vec<i32>>();
        let mut ret = (0..(mat.len() as i32)).collect::<Vec<i32>>();
        ret.sort_by_key(|&k| mat[k as usize]);

        ret[..(k as usize)].to_vec()
    }
}
```
