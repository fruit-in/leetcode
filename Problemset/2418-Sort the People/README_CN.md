# 2418. 按身高排序
给你一个字符串数组 `names` ，和一个由 **互不相同** 的正整数组成的数组 `heights` 。两个数组的长度均为 `n` 。

对于每个下标 `i`，`names[i]` 和 `heights[i]` 表示第 `i` 个人的名字和身高。

请按身高 **降序** 顺序返回对应的名字数组 `names` 。

#### 示例 1:
<pre>
<strong>输入:</strong> names = ["Mary","John","Emma"], heights = [180,165,170]
<strong>输出:</strong> ["Mary","Emma","John"]
<strong>解释:</strong> Mary 最高，接着是 Emma 和 John 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> names = ["Alice","Bob","Bob"], heights = [155,185,150]
<strong>输出:</strong> ["Bob","Alice","Bob"]
<strong>解释:</strong> 第一个 Bob 最高，然后是 Alice 和第二个 Bob 。
</pre>

#### 提示:
* `n == names.length == heights.length`
* <code>1 <= n <= 10<sup>3</sup></code>
* `1 <= names[i].length <= 20`
* <code>1 <= heights[i] <= 10<sup>5</sup></code>
* `names[i]` 由大小写英文字母组成
* `heights` 中的所有值互不相同

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people = names.iter().zip(heights.iter()).collect::<Vec<_>>();
        people.sort_unstable_by_key(|(_, &h)| -h);

        people.into_iter().map(|(n, _)| n.clone()).collect()
    }
}
```
