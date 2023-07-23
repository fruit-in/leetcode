# 455. 分发饼干
假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是，每个孩子最多只能给一块饼干。对每个孩子 i ，都有一个胃口值 g<sub>i</sub> ，这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j ，都有一个尺寸 s<sub>j</sub> 。如果 s<sub>j</sub> >= g<sub>i</sub> ，我们可以将这个饼干 j 分配给孩子 i ，这个孩子会得到满足。你的目标是尽可能满足越多数量的孩子，并输出这个最大数值。

#### 注意:
你可以假设胃口值为正。<br>
一个小朋友最多只能拥有一块饼干。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3], [1,1]
<strong>输出:</strong> 1
<strong>解释:</strong>
你有三个孩子和两块小饼干，3个孩子的胃口值分别是：1,2,3。
虽然你有两块小饼干，由于他们的尺寸都是1，你只能让胃口值是1的孩子满足。
所以你应该输出1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2], [1,2,3]
<strong>输出:</strong> 2
<strong>解释:</strong>
你有两个孩子和三块小饼干，2个孩子的胃口值分别是1,2。
你拥有的饼干数量和尺寸都足以让所有孩子满足。
所以你应该输出2.
</pre>

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} g
# @param {Integer[]} s
# @return {Integer}
def find_content_children(g, s)
    i, j = 0, 0
    ret = 0

    g.sort!
    s.sort!

    while i < g.length and j < s.length
        if s[j] >= g[i]
            ret += 1
            i += 1
        end
        j += 1
    end

    return ret
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut ret = 0;

        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                ret += 1;
                i += 1;
            }
            j += 1;
        }

        ret
    }
}
```
