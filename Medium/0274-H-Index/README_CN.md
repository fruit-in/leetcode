# 274. H 指数
给定一位研究者论文被引用次数的数组（被引用次数是非负整数）。编写一个方法，计算出研究者的 *h* 指数。

[h 指数的定义](https://baike.baidu.com/item/h-index/3991452?fr=aladdin)：h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）**总共**有 h 篇论文分别被引用了**至少** h 次。（其余的 *N - h* 篇论文每篇被引用次数 **不超过** *h* 次。）

例如：某人的 h 指数是 20，这表示他已发表的论文中，每篇被引用了至少 20 次的论文总共有 20 篇。

#### 示例:
<pre>
<strong>输入:</strong> citations = [3,0,6,1,5]
<strong>输出:</strong> 3
<strong>解释:</strong> 给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 3, 0, 6, 1, 5 次。
     由于研究者有 3 篇论文每篇 <strong>至少</strong> 被引用了 3 次，其余两篇论文每篇被引用 <strong>不多于</strong> 3 次，所以她的 <em>h</em> 指数是 3。
</pre>

#### 提示:
如果 *h* 有多种可能的值，*h* 指数是其中最大的那个。

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_unstable_by(|a, b| b.cmp(a));
        let mut h = 0;

        for i in 0..citations.len() {
            h = h.max(citations[i].min(i as i32 + 1));
        }

        h
    }
}
```

### 2. 计数
```Rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut counter = vec![0; citations.len() + 1];
        for &c in &citations {
            counter[(c as usize).min(citations.len())] += 1;
        }

        let mut s = 0;
        let mut i = counter.len() - 1;
        loop {
            s += counter[i];
            if s >= i {
                return i as i32;
            }
            i -= 1;
        }
    }
}
```
