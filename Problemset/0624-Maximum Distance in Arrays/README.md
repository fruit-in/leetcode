# 624. Maximum Distance in Arrays
You are given `m` `arrays`, where each array is sorted in **ascending order**.

You can pick up two integers from two different arrays (each array picks one) and calculate the distance. We define the distance between two integers `a` and `b` to be their absolute difference `|a - b|`.

Return *the maximum distance*.

#### Example 1:
<pre>
<strong>Input:</strong> arrays = [[1,2,3],[4,5],[1,2,3]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One way to reach the maximum distance 4 is to pick 1 in the first or third array and pick 5 in the second array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arrays = [[1],[1]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `m == arrays.length`
* <code>2 <= m <= 10<sup>5</sup></code>
* `1 <= arrays[i].length <= 500`
* <code>-10<sup>4</sup> <= arrays[i][j] <= 10<sup>4</sup></code>
* `arrays[i]` is sorted in **ascending order**.
* There will be at most <code>10<sup>5</sup></code> integers in all the arrays.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_num = arrays[0][0];
        let mut max_num = *arrays[0].last().unwrap();
        let mut ret = 0;

        for i in 1..arrays.len() {
            ret = ret
                .max(*arrays[i].last().unwrap() - min_num)
                .max(max_num - arrays[i][0]);
            min_num = min_num.min(arrays[i][0]);
            max_num = max_num.max(*arrays[i].last().unwrap());
        }

        ret
    }
}
```
