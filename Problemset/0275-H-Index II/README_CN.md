# 275. H指数 II
给定一位研究者论文被引用次数的数组（被引用次数是非负整数），数组已经按照**升序排列**。编写一个方法，计算出研究者的 *h* 指数。

[h 指数的定义](https://baike.baidu.com/item/h-index/3991452?fr=aladdin): “h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）**总共**有 h 篇论文分别被引用了**至少** h 次。（其余的 *N - h* 篇论文每篇被引用次数**不多于** *h* 次。）"

#### 示例:
<pre>
<strong>输入:</strong> citations = [0,1,3,5,6]
<strong>输出:</strong> 3
<strong>解释:</strong> 给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 0, 1, 3, 5, 6 次。
     由于研究者有 3 篇论文每篇<strong>至少</strong>被引用了 3 次，其余两篇论文每篇被引用<strong>不多于</strong> 3 次，所以她的 <em>h</em> 指数是 3。
</pre>

#### 说明:
如果 *h* 有多有种可能的值 ，*h* 指数是其中最大的那个。

#### 进阶:
* 这是 [H指数](https://leetcode-cn.com/problems/h-index/description/) 的延伸题目，本题中的 `citations` 数组是保证有序的。
* 你可以优化你的算法到对数时间复杂度吗？

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let mut l = 0;
        let mut r = len;
        let mut ret = 0;

        while l < r {
            let m = (l + r) / 2;

            if citations[m] as usize <= len - m {
                ret = ret.max(citations[m]);
                l = m + 1;
            } else {
                ret = ret.max((len - m) as i32);
                r = m;
            }
        }

        ret
    }
}
```
