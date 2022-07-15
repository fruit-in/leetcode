# 1860. 增长的内存泄露
给你两个整数 `memory1` 和 `memory2` 分别表示两个内存条剩余可用内存的位数。现在有一个程序每秒递增的速度消耗着内存。

在第 `i` 秒（秒数从 1 开始），有 `i` 位内存被分配到 **剩余内存较多** 的内存条（如果两者一样多，则分配到第一个内存条）。如果两者剩余内存都不足 `i` 位，那么程序将 **意外退出** 。

请你返回一个数组，包含 <code>[crashTime, memory1<sub>crash</sub>, memory2<sub>crash</sub>]</code> ，其中 `crashTime`是程序意外退出的时间（单位为秒）， <code>memory1<sub>crash</sub></code> 和 <code>memory2<sub>crash</sub></code> 分别是两个内存条最后剩余内存的位数。

#### 示例 1:
<pre>
<strong>输入:</strong> memory1 = 2, memory2 = 2
<strong>输出:</strong> [3,1,0]
<strong>解释:</strong> 内存分配如下：
- 第 1 秒，内存条 1 被占用 1 位内存。内存条 1 现在有 1 位剩余可用内存。
- 第 2 秒，内存条 2 被占用 2 位内存。内存条 2 现在有 0 位剩余可用内存。
- 第 3 秒，程序意外退出，两个内存条分别有 1 位和 0 位剩余可用内存。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> memory1 = 8, memory2 = 11
<strong>输出:</strong> [6,0,4]
<strong>解释:</strong> 内存分配如下：
- 第 1 秒，内存条 2 被占用 1 位内存，内存条 2 现在有 10 位剩余可用内存。
- 第 2 秒，内存条 2 被占用 2 位内存，内存条 2 现在有 8 位剩余可用内存。
- 第 3 秒，内存条 1 被占用 3 位内存，内存条 1 现在有 5 位剩余可用内存。
- 第 4 秒，内存条 2 被占用 4 位内存，内存条 2 现在有 4 位剩余可用内存。
- 第 5 秒，内存条 1 被占用 5 位内存，内存条 1 现在有 0 位剩余可用内存。
- 第 6 秒，程序意外退出，两个内存条分别有 0 位和 4 位剩余可用内存。
</pre>

#### 提示:
* <code>0 <= memory1, memory2 <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut bits = 1;
        let mut crash_time = 1;
        let mut memory1 = memory1;
        let mut memory2 = memory2;

        while memory1.max(memory2) >= bits {
            crash_time += 1;
            if memory1 >= memory2 {
                memory1 -= bits;
            } else {
                memory2 -= bits;
            }
            bits += 1;
        }

        vec![crash_time, memory1, memory2]
    }
}
```
