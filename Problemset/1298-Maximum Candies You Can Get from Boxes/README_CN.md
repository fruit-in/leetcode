# 1298. 你能从盒子里获得的最大糖果数
给你 `n` 个盒子，每个盒子的格式为 `[status, candies, keys, containedBoxes]` ，其中：

* 状态字 `status[i]`：整数，如果 `box[i]` 是开的，那么是 **1** ，否则是 **0** 。
* 糖果数 `candies[i]`: 整数，表示 `box[i]` 中糖果的数目。
* 钥匙 `keys[i]`：数组，表示你打开 `box[i]` 后，可以得到一些盒子的钥匙，每个元素分别为该钥匙对应盒子的下标。
* 内含的盒子 `containedBoxes[i]`：整数，表示放在 `box[i]` 里的盒子所对应的下标。

给你一个 `initialBoxes` 数组，表示你现在得到的盒子，你可以获得里面的糖果，也可以用盒子里的钥匙打开新的盒子，还可以继续探索从这个盒子里找到的其他盒子。

请你按照上述规则，返回可以获得糖果的 **最大数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> status = [1,0,1,0], candies = [7,5,4,100], keys = [[],[],[1],[]], containedBoxes = [[1,2],[3],[],[]], initialBoxes = [0]
<strong>输出:</strong> 16
<strong>解释:</strong>
一开始你有盒子 0 。你将获得它里面的 7 个糖果和盒子 1 和 2。
盒子 1 目前状态是关闭的，而且你还没有对应它的钥匙。所以你将会打开盒子 2 ，并得到里面的 4 个糖果和盒子 1 的钥匙。
在盒子 1 中，你会获得 5 个糖果和盒子 3 ，但是你没法获得盒子 3 的钥匙所以盒子 3 会保持关闭状态。
你总共可以获得的糖果数目 = 7 + 4 + 5 = 16 个。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> status = [1,0,0,0,0,0], candies = [1,1,1,1,1,1], keys = [[1,2,3,4,5],[],[],[],[],[]], containedBoxes = [[1,2,3,4,5],[],[],[],[],[]], initialBoxes = [0]
<strong>输出:</strong> 6
<strong>Explanation:</strong>
你一开始拥有盒子 0 。打开它你可以找到盒子 1,2,3,4,5 和它们对应的钥匙。
打开这些盒子，你将获得所有盒子的糖果，所以总糖果数为 6 个。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> status = [1,1,1], candies = [100,1,100], keys = [[],[0,2],[]], containedBoxes = [[],[],[]], initialBoxes = [1]
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> status = [1], candies = [100], keys = [[]], containedBoxes = [[]], initialBoxes = []
<strong>输出:</strong> 0
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> status = [1,1,1], candies = [2,3,2], keys = [[],[],[]], containedBoxes = [[],[],[]], initialBoxes = [2,1,0]
<strong>输出:</strong> 7
</pre>

#### 提示:
* `1 <= status.length <= 1000`
* `status.length == candies.length == keys.length == containedBoxes.length == n`
* `status[i]` 要么是 `0` 要么是 `1` 。
* `1 <= candies[i] <= 1000`
* `0 <= keys[i].length <= status.length`
* `0 <= keys[i][j] < status.length`
* `keys[i]` 中的值都是互不相同的。
* `0 <= containedBoxes[i].length <= status.length`
* `0 <= containedBoxes[i][j] < status.length`
* `containedBoxes[i]` 中的值都是互不相同的。
* 每个盒子最多被一个盒子包含。
* `0 <= initialBoxes.length <= status.length`
* `0 <= initialBoxes[i] < status.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut closed = HashSet::new();
        let mut opened = vec![];
        let mut used = HashSet::new();
        let mut ret = 0;

        for b in initial_boxes {
            if status[b as usize] == 1 && !used.contains(&b) {
                opened.push(b);
                used.insert(b);
            } else if status[b as usize] == 0 {
                closed.insert(b);
            }
        }

        while let Some(b) = opened.pop() {
            ret += candies[b as usize];

            for &k in &keys[b as usize] {
                status[k as usize] = 1;
                if closed.remove(&k) && !used.contains(&k) {
                    opened.push(k);
                    used.insert(k);
                }
            }

            for &cb in &contained_boxes[b as usize] {
                if status[cb as usize] == 1 && !used.contains(&cb) {
                    opened.push(cb);
                    used.insert(cb);
                } else if status[cb as usize] == 0 {
                    closed.insert(cb);
                }
            }
        }

        ret
    }
}
```
