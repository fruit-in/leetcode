# 2363. 合并相似的物品
给你两个二维整数数组 `items1` 和 `items2` ，表示两个物品集合。每个数组 `items` 有以下特质：

* <code>items[i] = [value<sub>i</sub>, weight<sub>i</sub>]</code> 其中 <code>value<sub>i</sub></code> 表示第 `i` 件物品的 **价值** ，<code>weight<sub>i</sub></code> 表示第 `i` 件物品的 **重量** 。
* `items` 中每件物品的价值都是 唯一的 。

请你返回一个二维数组 `ret`，其中 <code>ret[i] = [value<sub>i</sub>, weight<sub>i</sub>]</code>， <code>weight<sub>i</sub></code> 是所有价值为 <code>value<sub>i</sub></code> 物品的 **重量之和** 。

**注意**：`ret` 应该按价值 **升序** 排序后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> items1 = [[1,1],[4,5],[3,8]], items2 = [[3,1],[1,5]]
<strong>输出:</strong> [[1,6],[3,9],[4,5]]
<strong>解释:</strong>
value = 1 的物品在 items1 中 weight = 1 ，在 items2 中 weight = 5 ，总重量为 1 + 5 = 6 。
value = 3 的物品再 items1 中 weight = 8 ，在 items2 中 weight = 1 ，总重量为 8 + 1 = 9 。
value = 4 的物品在 items1 中 weight = 5 ，总重量为 5 。
所以，我们返回 [[1,6],[3,9],[4,5]] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> items1 = [[1,1],[3,2],[2,3]], items2 = [[2,1],[3,2],[1,3]]
<strong>输出:</strong> [[1,4],[2,4],[3,4]]
<strong>解释:</strong>
value = 1 的物品在 items1 中 weight = 1 ，在 items2 中 weight = 3 ，总重量为 1 + 3 = 4 。
value = 2 的物品在 items1 中 weight = 3 ，在 items2 中 weight = 1 ，总重量为 3 + 1 = 4 。
value = 3 的物品在 items1 中 weight = 2 ，在 items2 中 weight = 2 ，总重量为 2 + 2 = 4 。
所以，我们返回 [[1,4],[2,4],[3,4]] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> items1 = [[1,3],[2,2]], items2 = [[7,1],[2,2],[1,4]]
<strong>输出:</strong> [[1,7],[2,4],[7,1]]
<strong>解释:</strong>
value = 1 的物品在 items1 中 weight = 3 ，在 items2 中 weight = 4 ，总重量为 3 + 4 = 7 。
value = 2 的物品在 items1 中 weight = 2 ，在 items2 中 weight = 2 ，总重量为 2 + 2 = 4 。
value = 7 的物品在 items2 中 weight = 1 ，总重量为 1 。
所以，我们返回 [[1,7],[2,4],[7,1]] 。
</pre>

#### 提示:
* `1 <= items1.length, items2.length <= 1000`
* `items1[i].length == items2[i].length == 2`
* <code>1 <= value<sub>i</sub>, weight<sub>i</sub> <= 1000</code>
* `items1` 中每个 <code>value<sub>i</sub></code> 都是 **唯一的** 。
* `items2` 中每个 <code>value<sub>i</sub></code> 都是 **唯一的** 。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut items = HashMap::new();

        for item in items1.iter().chain(items2.iter()) {
            items
                .entry(item[0])
                .and_modify(|w| *w += item[1])
                .or_insert(item[1]);
        }

        let mut ret = items
            .into_iter()
            .map(|(v, w)| vec![v, w])
            .collect::<Vec<_>>();
        ret.sort_unstable();

        ret
    }
}
```
