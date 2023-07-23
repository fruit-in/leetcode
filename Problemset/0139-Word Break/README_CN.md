# 139. 单词拆分
给定一个**非空**字符串 *s* 和一个包含**非空**单词的列表 *wordDict*，判定 *s* 是否可以被空格拆分为一个或多个在字典中出现的单词。

#### 说明:
* 拆分时可以重复使用字典中的单词。
* 你可以假设字典中没有重复的单词。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leetcode", wordDict = ["leet","code"]
<strong>输出:</strong> true
<strong>解释:</strong> 返回 true 因为 "leetcode" 可以被拆分成 "leet code"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "applepenapple", wordDict = ["apple","pen"]
<strong>输出:</strong> true
<strong>解释:</strong> 返回 true 因为 "applepenapple" 可以被拆分成 "apple pen apple"。
     注意你可以重复使用字典中的单词。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
<strong>输出:</strong> false
</pre>

## 题解 (Python)

### 1. 动态规划
```Python
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        dp = [True] + [False] * len(s)

        for i in range(1, len(s) + 1):
            for j in range(i):
                if dp[j] and s[j:i] in wordDict:
                    dp[i] = True
                    break

        return dp[-1]
```

## 题解 (Ruby)

### 1. 动态规划
```Ruby
# @param {String} s
# @param {String[]} word_dict
# @return {Boolean}
def word_break(s, word_dict)
  dp = [true] + [false] * s.size

  (1..s.size).each do |i|
    (0...i).each do |j|
      if dp[j] && word_dict.include?(s[j...i])
        dp[i] = true
        break
      end
    end
  end

  dp[s.size]
end
```
