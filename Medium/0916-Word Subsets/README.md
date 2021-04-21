# 916. Word Subsets
We are given two arrays `A` and `B` of words.  Each word is a string of lowercase letters.

Now, say that word `b` is a subset of word `a` if every letter in `b` occurs in `a`, **including multiplicity**.  For example, `"wrr"` is a subset of `"warrior"`, but is not a subset of `"world"`.

Now say a word `a` from `A` is *universal* if for every `b` in `B`, `b` is a subset of `a`.

Return a list of all universal words in `A`.  You can return the words in any order.

#### Example 1:
<pre>
<strong>Input:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["e","o"]
<strong>Output:</strong> ["facebook","google","leetcode"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["l","e"]
<strong>Output:</strong> ["apple","google","leetcode"]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["e","oo"]
<strong>Output:</strong> ["facebook","google"]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["lo","eo"]
<strong>Output:</strong> ["google","leetcode"]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> A = ["amazon","apple","facebook","google","leetcode"], B = ["ec","oc","ceo"]
<strong>Output:</strong> ["facebook","leetcode"]
</pre>

#### Note:
1. `1 <= A.length, B.length <= 10000`
2. `1 <= A[i].length, B[i].length <= 10`
3. `A[i]` and `B[i]` consist only of lowercase letters.
4. All words in `A[i]` are unique: there isn't `i != j` with `A[i] == A[j]`.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
