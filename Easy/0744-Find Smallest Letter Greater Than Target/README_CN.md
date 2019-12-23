# 744. 寻找比目标字母大的最小字母
给定一个只包含小写字母的有序数组```letters``` 和一个目标字母 ```target```，寻找有序数组里面比目标字母大的最小字母。

数组里字母的顺序是循环的。举个例子，如果目标字母```target = 'z'``` 并且有序数组为 ```letters = ['a', 'b']```，则答案返回 ```'a'```。

#### 示例:
<pre>
<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "a"
<strong>输出:</strong> "c"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "c"
<strong>输出:</strong> "f"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "d"
<strong>输出:</strong> "f"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "g"
<strong>输出:</strong> "j"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "j"
<strong>输出:</strong> "c"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "k"
<strong>输出:</strong> "c"
</pre>

#### 注:
1. ```letters```长度范围在```[2, 10000]```区间内。
2. ```letters``` 仅由小写字母组成，最少包含两个不同的字母。
3. 目标字母```target``` 是一个小写字母。

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &ch in &letters {
            if ch > target {
                return ch;
            }
        }
        letters[0]
    }
}
```

### 2. 二分查找
```Rust
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match letters.binary_search(&target) {
            Ok(i) => letters[(i + 1) % letters.len()],
            Err(i) => letters[i % letters.len()],
        }
    }
}
```
