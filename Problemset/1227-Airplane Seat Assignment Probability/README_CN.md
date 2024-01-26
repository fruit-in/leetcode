# 1227. 飞机座位分配概率
有 `n` 位乘客即将登机，飞机正好有 `n` 个座位。第一位乘客的票丢了，他随便选了一个座位坐下。

剩下的乘客将会：

* 如果他们自己的座位还空着，就坐到自己的座位上，
* 当他们自己的座位被占用时，随机选择其他座位

第 `n` 位乘客坐在自己的座位上的概率是多少？

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1.00000
<strong>解释:</strong> 第一个人只会坐在自己的位置上。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 0.50000
<strong>解释:</strong> 在第一个人选好座位坐下后，第二个人坐在自己的座位上的概率是 0.5。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        match n {
            1 => 1.0,
            _ => 0.5,
        }
    }
}
```
