# 2558. 从数量最多的堆取走礼物
给你一个整数数组 `gifts` ，表示各堆礼物的数量。每一秒，你需要执行以下操作：

* 选择礼物数量最多的那一堆。
* 如果不止一堆都符合礼物数量最多，从中选择任一堆即可。
* 选中的那一堆留下平方根数量的礼物（向下取整），取走其他的礼物。

返回在 `k` 秒后剩下的礼物数量。

#### 示例 1:
<pre>
<strong>输入:</strong> gifts = [25,64,9,4,100], k = 4
<strong>输出:</strong> 29
<strong>解释:</strong>
按下述方式取走礼物：
- 在第一秒，选中最后一堆，剩下 10 个礼物。
- 接着第二秒选中第二堆礼物，剩下 8 个礼物。
- 然后选中第一堆礼物，剩下 5 个礼物。
- 最后，再次选中最后一堆礼物，剩下 3 个礼物。
最后剩下的礼物数量分别是 [5,8,9,4,3] ，所以，剩下礼物的总数量是 29 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> gifts = [1,1,1,1], k = 4
<strong>输出:</strong> 4
<strong>解释:</strong>
在本例中，不管选中哪一堆礼物，都必须剩下 1 个礼物。
也就是说，你无法获取任一堆中的礼物。
所以，剩下礼物的总数量是 4 。
</pre>

#### 提示:
* <code>1 <= gifts.length <= 10<sup>3</sup></code>
* <code>1 <= gifts[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>3</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts = BinaryHeap::from(gifts);

        for _ in 0..k {
            let mut x = gifts.peek_mut().unwrap();
            *x = (*x as f64).sqrt() as i32;
        }

        gifts.iter().map(|&x| x as i64).sum()
    }
}
```
