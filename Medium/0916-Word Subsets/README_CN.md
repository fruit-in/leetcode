# 916. 单词子集
我们给出两个单词数组 `A` 和 `B`。每个单词都是一串小写字母。

现在，如果 `b` 中的每个字母都出现在 `a` 中，**包括重复出现的字母**，那么称单词 `b` 是单词 `a` 的子集。 例如，“wrr” 是 “warrior” 的子集，但不是 “world” 的子集。

如果对 `B` 中的每一个单词 `b`，`b` 都是 `a` 的子集，那么我们称 `A` 中的单词 `a` 是*通用的*。

你可以按任意顺序以列表形式返回 `A` 中所有的通用单词。

#### 示例 1:
<pre>
<strong>输入:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["e","o"]
<strong>输出:</strong> ["facebook","google","leetcode"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["l","e"]
<strong>输出:</strong> ["apple","google","leetcode"]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["e","oo"]
<strong>输出:</strong> ["facebook","google"]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["lo","eo"]
<strong>输出:</strong> ["google","leetcode"]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["ec","oc","ceo"]
<strong>输出:</strong> ["facebook","leetcode"]
</pre>

#### 提示:
1. `1 <= A.length, B.length <= 10000`
2. `1 <= A[i].length, B[i].length <= 10`
3. `A[i]` 和 `B[i]` 只由小写字母组成。
4. `A[i]` 中所有的单词都是独一无二的，也就是说不存在 `i != j` 使得 `A[i] == A[j]`。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String[]} a
# @param {String[]} b
# @return {String[]}
def word_subsets(a, b)
  count = [0] * 26
  b.each do |sub|
    count_ = count_chars(sub)
    (0...26).each do |i|
      count[i] = [count[i], count_[i]].max
    end
  end

  a.filter { |word| count_chars(word).zip(count).all? { |c| c[0] >= c[1] } }
end

# @param {String} s
# @return {Integer[]}
def count_chars(s)
  count = [0] * 26
  s.each_byte { |c| count[c - 97] += 1 }

  count
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut count = [0; 26];
        for sub in b {
            let count_ = Self::count_chars(&sub);
            for i in 0..26 {
                count[i] = count[i].max(count_[i]);
            }
        }

        a.into_iter()
            .filter(|word| {
                Self::count_chars(&word)
                    .iter()
                    .zip(count.iter())
                    .all(|(x, y)| x >= y)
            })
            .collect()
    }

    fn count_chars(s: &str) -> [i32; 26] {
        let mut count = [0; 26];
        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }

        count
    }
}
```
