# 986. Interval List Intersections
Given two lists of **closed** intervals, each list of intervals is pairwise disjoint and in sorted order.

Return the intersection of these two interval lists.

*(Formally, a closed interval ```[a, b]``` (with ```a <= b```) denotes the set of real numbers ```x``` with ```a <= x <= b```.  The intersection of two closed intervals is a set of real numbers that is either empty, or can be represented as a closed interval.  For example, the intersection of [1, 3] and [2, 4] is [2, 3].)*

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/01/30/interval1.png)
<pre>
<strong>Input:</strong> A = [[0,2],[5,10],[13,23],[24,25]], B = [[1,5],[8,12],[15,24],[25,26]]
<strong>Output:</strong> [[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]]
<strong>Reminder:</strong> The inputs and the desired output are lists of Interval objects, and not arrays or lists.
</pre>

#### Note:
1. ```0 <= A.length < 1000```
2. ```0 <= B.length < 1000```
3. ```0 <= A[i].start, A[i].end, B[i].start, B[i].end < 10^9```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();

        while i < a.len() && j < b.len() {
            let max_l = a[i][0].max(b[j][0]);
            let min_r = a[i][1].min(b[j][1]);

            if min_r >= max_l {
                ret.push(vec![max_l, min_r]);
            }

            if min_r == a[i][1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        ret
    }
}
```
