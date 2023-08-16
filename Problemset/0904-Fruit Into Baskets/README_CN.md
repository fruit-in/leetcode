# 904. 水果成篮
你正在探访一家农场，农场从左到右种植了一排果树。这些树用一个整数数组 `fruits` 表示，其中 `fruits[i]` 是第 `i` 棵树上的水果 **种类** 。

你想要尽可能多地收集水果。然而，农场的主人设定了一些严格的规矩，你必须按照要求采摘水果：

* 你只有 **两个** 篮子，并且每个篮子只能装 **单一类型** 的水果。每个篮子能够装的水果总量没有限制。
* 你可以选择任意一棵树开始采摘，你必须从 **每棵** 树（包括开始采摘的树）上 **恰好摘一个水果** 。采摘的水果应当符合篮子中的水果类型。每采摘一次，你将会向右移动到下一棵树，并继续采摘。
* 一旦你走到某棵树前，但水果不符合篮子的水果类型，那么就必须停止采摘。

给你一个整数数组 `fruits` ，返回你可以收集的水果的 **最大** 数目。

#### 示例 1:
<pre>
<strong>输入:</strong> fruits = [1,2,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 可以采摘全部 3 棵树。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> fruits = [0,1,2,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 可以采摘 [1,2,2] 这三棵树。
如果从第一棵树开始采摘，则只能采摘 [0,1] 这两棵树。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> fruits = [1,2,3,2,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 可以采摘 [2,3,2,2] 这四棵树。
如果从第一棵树开始采摘，则只能采摘 [1,2] 这两棵树。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> fruits = [3,3,3,1,2,1,1,2,3,3,4]
<strong>输出:</strong> 5
<strong>解释:</strong> 可以采摘 [1,2,1,1,2] 这五棵树。
</pre>

#### 提示:
* <code>1 <= fruits.length <= 10<sup>5</sup></code>
* `0 <= fruits[i] < fruits.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count = HashMap::from([(fruits[0], 1)]);
        let mut r = 0;
        let mut ret = 0;

        for l in 0..fruits.len() {
            while count.len() <= 2 {
                ret = ret.max(r - l + 1);

                if r + 1 == fruits.len() {
                    break;
                }
                r += 1;
                count.entry(fruits[r]).and_modify(|c| *c += 1).or_insert(1);
            }

            if count[&fruits[l]] == 1 {
                count.remove(&fruits[l]);
            } else {
                *count.get_mut(&fruits[l]).unwrap() -= 1;
            }
        }

        ret as i32
    }
}
```
