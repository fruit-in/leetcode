# 1220. 统计元音字母序列的数目
给你一个整数 ```n```，请你帮忙统计一下我们可以按下述规则形成多少个长度为 ```n``` 的字符串：
* 字符串中的每个字符都应当是小写元音字母（```'a'```, ```'e'```, ```'i'```, ```'o'```, ```'u'```）
* 每个元音 ```'a'``` 后面都只能跟着 ```'e'```
* 每个元音 ```'e'``` 后面只能跟着 ```'a'``` 或者是 ```'i'```
* 每个元音 ```'i'``` 后面 **不能** 再跟着另一个 ```'i'```
* 每个元音 ```'o'``` 后面只能跟着 ```'i'``` 或者是 ```'u'```
* 每个元音 ```'u'``` 后面只能跟着 ```'a'```

由于答案可能会很大，所以请你返回 模 ```10^9 + 7``` 之后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 5
<strong>解释:</strong> 所有可能的字符串分别是："a", "e", "i" , "o" 和 "u"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 10
<strong>解释:</strong> 所有可能的字符串分别是："ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" 和 "ua"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 68
</pre>

#### 提示:
* ```1 <= n <= 2 * 10^4```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut end = vec![1 as u32; 5];

        for _ in 1..n {
            let mut tmp = Vec::new();
            tmp.push((end[1] + end[2] + end[4]) % 1_000_000_007);
            tmp.push((end[0] + end[2]) % 1_000_000_007);
            tmp.push((end[1] + end[3]) % 1_000_000_007);
            tmp.push((end[2]) % 1_000_000_007);
            tmp.push((end[2] + end[3]) % 1_000_000_007);
            end = tmp;
        }

        (end.iter().sum::<u32>() % 1_000_000_007) as i32
    }
}
```
