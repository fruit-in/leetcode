# 1743. 从相邻元素对还原数组
存在一个由 `n` 个不同元素组成的整数数组 `nums` ，但你已经记不清具体内容。好在你还记得 `nums` 中的每一对相邻元素。

给你一个二维整数数组 `adjacentPairs` ，大小为 `n - 1` ，其中每个 <code>adjacentPairs[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示元素 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 在 `nums` 中相邻。

题目数据保证所有由元素 `nums[i]` 和 `nums[i+1]` 组成的相邻元素对都存在于 `adjacentPairs` 中，存在形式可能是 `[nums[i], nums[i+1]]` ，也可能是 `[nums[i+1], nums[i]]` 。这些相邻元素对可以 **按任意顺序** 出现。

返回 **原始数组** `nums` 。如果存在多种解答，返回 **其中任意一个** 即可。

#### 示例 1:
<pre>
<strong>输入:</strong> adjacentPairs = [[2,1],[3,4],[3,2]]
<strong>输出:</strong> [1,2,3,4]
<strong>解释:</strong> 数组的所有相邻元素对都在 adjacentPairs 中。
特别要注意的是，adjacentPairs[i] 只表示两个元素相邻，并不保证其 左-右 顺序。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> adjacentPairs = [[4,-2],[1,4],[-3,1]]
<strong>输出:</strong> [-2,4,1,-3]
<strong>解释:</strong> 数组中可能存在负数。
另一种解答是 [-3,1,4,-2] ，也会被视作正确答案。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> adjacentPairs = [[100000,-100000]]
<strong>输出:</strong> [100000,-100000]
</pre>

#### 提示:
* `nums.length == n`
* `adjacentPairs.length == n - 1`
* `adjacentPairs[i].length == 2`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i], u<sub>i</sub>, v<sub>i</sub> <= 10<sup>5</sup></code>
* 题目数据保证存在一些以 `adjacentPairs` 作为元素对的数组 `nums`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjacent = HashMap::new();
        let mut ret = vec![];

        for pair in &adjacent_pairs {
            adjacent.entry(pair[0]).or_insert(vec![]).push(pair[1]);
            adjacent.entry(pair[1]).or_insert(vec![]).push(pair[0]);
        }

        let mut curr = *adjacent.iter().find(|(_, v)| v.len() == 1).unwrap().0;
        ret.push(curr);

        for i in 0..adjacent_pairs.len() {
            curr = match &adjacent.get(&curr).unwrap()[..] {
                &[x] => x,
                &[x, y] if x != ret[i - 1] => x,
                &[x, y] => y,
                _ => 0,
            };
            ret.push(curr);
        }

        ret
    }
}
```
