# 986. 区间列表的交集
给定两个由一些**闭区间**组成的列表，每个区间列表都是成对不相交的，并且已经排序。

返回这两个区间列表的交集。

*（形式上，闭区间 ```[a, b]```（其中 ```a <= b```）表示实数 ```x``` 的集合，而 ```a <= x <= b```。两个闭区间的交集是一组实数，要么为空集，要么为闭区间。例如，[1, 3] 和 [2, 4] 的交集为 [2, 3]。）*

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/02/02/interval1.png)
<pre>
<strong>输入:</strong> A = [[0,2],[5,10],[13,23],[24,25]], B = [[1,5],[8,12],[15,24],[25,26]]
<strong>输出:</strong> [[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]]
<strong>注意:</strong> 输入和所需的输出都是区间对象组成的列表，而不是数组或列表。
</pre>

#### 提示:
1. ```0 <= A.length < 1000```
2. ```0 <= B.length < 1000```
3. ```0 <= A[i].start, A[i].end, B[i].start, B[i].end < 10^9```

## 题解 (Rust)

### 1. 题解
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
