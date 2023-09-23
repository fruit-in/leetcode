# 2251. 花期内花的数目
给你一个下标从 **0** 开始的二维整数数组 `flowers` ，其中 <code>flowers[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 表示第 `i` 朵花的 **花期** 从 <code>start<sub>i</sub></code> 到 <code>end<sub>i</sub></code> （都 **包含**）。同时给你一个下标从 **0** 开始大小为 `n` 的整数数组 `persons` ，`persons[i]` 是第 `i` 个人来看花的时间。

请你返回一个大小为 `n` 的整数数组 `answer` ，其中 `answer[i]`是第 `i` 个人到达时在花期内花的 **数目** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/02/ex1new.jpg)
<pre>
<strong>输入:</strong> flowers = [[1,6],[3,7],[9,12],[4,13]], poeple = [2,3,7,11]
<strong>输出:</strong> [1,2,2,2]
<strong>解释:</strong> 上图展示了每朵花的花期时间，和每个人的到达时间。
对每个人，我们返回他们到达时在花期内花的数目。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/02/ex2new.jpg)
<pre>
<strong>输入:</strong> flowers = [[1,10],[3,3]], poeple = [3,3,2]
<strong>输出:</strong> [2,2,1]
<strong>解释:</strong> 上图展示了每朵花的花期时间，和每个人的到达时间。
对每个人，我们返回他们到达时在花期内花的数目。
</pre>

#### 提示:
* <code>1 <= flowers.length <= 5 * 10<sup>4</sup></code>
* `flowers[i].length == 2`
* <code>1 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= people.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= people[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut flowers = flowers;
        let mut people = people
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect::<Vec<_>>();
        let mut flowers_heap = BinaryHeap::new();
        let mut i = 0;
        let mut answer = vec![0; people.len()];

        flowers.sort_unstable();
        people.sort_unstable();

        for &(p, j) in &people {
            while let Some(&Reverse(end)) = flowers_heap.peek() {
                if end >= p {
                    break;
                }

                flowers_heap.pop();
            }

            while i < flowers.len() && flowers[i][0] <= p {
                if flowers[i][1] >= p {
                    flowers_heap.push(Reverse(flowers[i][1]));
                }

                i += 1;
            }

            answer[j] = flowers_heap.len() as i32;
        }

        answer
    }
}
```
