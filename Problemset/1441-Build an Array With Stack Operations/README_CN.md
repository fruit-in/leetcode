# 1441. 用栈操作构建数组
给你一个目标数组 `target` 和一个整数 `n`。每次迭代，需要从  `list = {1,2,3..., n}` 中依序读取一个数字。

请使用下述操作来构建目标数组 `target` ：
* **Push**：从 `list` 中读取一个新元素， 并将其推入数组中。
* **Pop**：删除数组中的最后一个元素。
* 如果目标数组构建完成，就停止读取更多元素。

题目数据保证目标数组严格递增，并且只包含 `1` 到 `n` 之间的数字。

请返回构建目标数组所用的操作序列。

题目数据保证答案是唯一的。

#### 示例 1:
<pre>
<strong>输入:</strong> target = [1,3], n = 3
<strong>输出:</strong> ["Push","Push","Pop","Push"]
<strong>解释:</strong>
读取 1 并自动推入数组 -> [1]
读取 2 并自动推入数组，然后删除它 -> [1]
读取 3 并自动推入数组 -> [1,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = [1,2,3], n = 3
<strong>输出:</strong> ["Push","Push","Push"]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> target = [1,2], n = 4
<strong>输出:</strong> ["Push","Push"]
<strong>解释:</strong> 只需要读取前 2 个数字就可以停止。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> target = [2,3,4], n = 4
<strong>输出:</strong> ["Push","Pop","Push","Push","Push"]
</pre>

#### 提示:
* `1 <= target.length <= 100`
* `1 <= target[i] <= 100`
* `1 <= n <= 100`
* `target` 是严格递增的

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut i = 1;
        let mut ret = vec![];

        for num in target {
            for _ in 0..(num - i) {
                ret.push("Push".to_string());
                ret.push("Pop".to_string());
            }
            ret.push("Push".to_string());
            i = num + 1;
        }

        ret
    }
}
```
