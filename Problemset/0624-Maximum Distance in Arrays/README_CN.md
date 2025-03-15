# 624. 数组列表中的最大距离
给定 `m` 个数组，每个数组都已经按照升序排好序了。

现在你需要从两个不同的数组中选择两个整数（每个数组选一个）并且计算它们的距离。两个整数 `a` 和 `b` 之间的距离定义为它们差的绝对值 `|a-b|` 。

返回最大距离。

#### 示例 1:
<pre>
<strong>输入:</strong> arrays = [[1,2,3],[4,5],[1,2,3]]
<strong>输出:</strong> 4
<strong>解释:</strong>
一种得到答案 4 的方法是从第一个数组或者第三个数组中选择 1，同时从第二个数组中选择 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arrays = [[1],[1]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `m == arrays.length`
* <code>2 <= m <= 10<sup>5</sup></code>
* `1 <= arrays[i].length <= 500`
* <code>-10<sup>4</sup> <= arrays[i][j] <= 10<sup>4</sup></code>
* `arrays[i]` 以 **升序** 排序。
* 所有数组中最多有 <code>10<sup>5</sup></code> 个整数。

## 题解 (Rust)

### 1. 题解
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
