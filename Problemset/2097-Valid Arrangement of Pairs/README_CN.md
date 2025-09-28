# 2097. 合法重新排列数对
给你一个下标从 **0** 开始的二维整数数组 `pairs` ，其中 <code>pairs[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 。如果 `pairs` 的一个重新排列，满足对每一个下标 `i` （ `1 <= i < pairs.length` ）都有 <code>end<sub>i-1</sub> == start<sub>i</sub></code> ，那么我们就认为这个重新排列是 `pairs` 的一个 **合法重新排列** 。

请你返回 **任意一个** `pairs` 的合法重新排列。

**注意：**数据保证至少存在一个 `pairs` 的合法重新排列。

#### 示例 1:
<pre>
<strong>输入:</strong> pairs = [[5,1],[4,5],[11,9],[9,4]]
<strong>输出:</strong> [[11,9],[9,4],[4,5],[5,1]]
<strong>解释:</strong>
输出的是一个合法重新排列，因为每一个 endi-1 都等于 starti 。
end0 = 9 == 9 = start1
end1 = 4 == 4 = start2
end2 = 5 == 5 = start3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pairs = [[1,3],[3,2],[2,1]]
<strong>输出:</strong> [[1,3],[3,2],[2,1]]
<strong>解释:</strong>
输出的是一个合法重新排列，因为每一个 endi-1 都等于 starti 。
end0 = 3 == 3 = start1
end1 = 2 == 2 = start2
重新排列后的数组 [[2,1],[1,3],[3,2]] 和 [[3,2],[2,1],[1,3]] 都是合法的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> pairs = [[1,2],[1,3],[2,1]]
<strong>输出:</strong> [[1,2],[2,1],[1,3]]
<strong>解释:</strong>
输出的是一个合法重新排列，因为每一个 endi-1 都等于 starti 。
end0 = 2 == 2 = start1
end1 = 1 == 1 = start2
</pre>

#### 提示:
* <code>1 <= pairs.length <= 10<sup>5</sup></code>
* `pairs[i].length == 2`
* <code>0 <= start<sub>i</sub>, end<sub>i</sub> <= 10<sup>9</sup></code>
* <code>start<sub>i</sub> != end<sub>i</sub></code>
* `pairs` 中不存在一模一样的数对。
* 至少 **存在** 一个合法的 `pairs` 重新排列。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut outdegree = HashMap::new();
        let mut indegree = HashMap::new();
        let mut unused = HashMap::new();
        let mut stack = vec![];
        let mut curr = pairs[0][0];
        let mut path = vec![];
        let mut ret = vec![];

        for pair in &pairs {
            *outdegree.entry(pair[0]).or_insert(0) += 1;
            *indegree.entry(pair[1]).or_insert(0) += 1;
            unused.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        }

        for pair in &pairs {
            if outdegree[&pair[0]] == *indegree.get(&pair[0]).unwrap_or(&0) + 1 {
                curr = pair[0];
                break;
            }
        }

        while curr != -1 {
            if !unused.get(&curr).unwrap_or(&vec![]).is_empty() {
                stack.push(curr);
                curr = unused.get_mut(&curr).unwrap().pop().unwrap();
            } else {
                path.push(curr);
                curr = stack.pop().unwrap_or(-1);
            }
        }

        for i in (1..path.len()).rev() {
            ret.push(vec![path[i], path[i - 1]]);
        }

        ret
    }
}
```
