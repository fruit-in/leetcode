# 292. Nim 游戏
你和你的朋友，两个人一起玩 [Nim 游戏](https://baike.baidu.com/item/Nim%E6%B8%B8%E6%88%8F/6737105)：桌子上有一堆石头，每次你们轮流拿掉 1 - 3 块石头。 拿掉最后一块石头的人就是获胜者。你作为先手。

你们是聪明人，每一步都是最优解。 编写一个函数，来判断你是否可以在给定石头数量的情况下赢得游戏。

#### 示例:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> false
<strong>解释:</strong> 如果堆中有 4 块石头，那么你永远不会赢得比赛；
     因为无论你拿走 1 块、2 块 还是 3 块石头，最后一块石头总是会被你的朋友拿走。
</pre>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @return {Boolean}
def can_win_nim(n)
    return n % 4 != 0
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}
```
