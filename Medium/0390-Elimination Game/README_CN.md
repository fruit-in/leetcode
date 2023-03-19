# 390. 消除游戏
列表 `arr` 由在范围 `[1, n]` 中的所有整数组成，并按严格递增排序。请你对 `arr` 应用下述算法：

* 从左到右，删除第一个数字，然后每隔一个数字删除一个，直到到达列表末尾。
* 重复上面的步骤，但这次是从右到左。也就是，删除最右侧的数字，然后剩下的数字每隔一个删除一个。
* 不断重复这两步，从左到右和从右到左交替进行，直到只剩下一个数字。

给你整数 `n` ，返回 `arr` 最后剩下的数字。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 9
<strong>输出:</strong> 6
<strong>解释:</strong>
arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]
arr = [2, 4, 6, 8]
arr = [2, 6]
arr = [6]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * Self::last_remaining_rev(n / 2)
    }

    pub fn last_remaining_rev(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * Self::last_remaining(n / 2) - 1 + n % 2
    }
}
```
