# 1528. 重新排列字符串
给你一个字符串 `s` 和一个 **长度相同** 的整数数组 `indices` 。

请你重新排列字符串 `s` ，其中第 `i` 个字符需要移动到 `indices[i]` 指示的位置。

返回重新排列后的字符串。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/26/q1.jpg)
<pre>
<strong>输入:</strong> s = "codeleet", indices = [4,5,6,7,0,2,1,3]
<strong>输出:</strong> "leetcode"
<strong>解释:</strong> 如图所示，"codeleet" 重新排列后变为 "leetcode" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abc", indices = [0,1,2]
<strong>输出:</strong> "abc"
<strong>解释:</strong> 重新排列后，每个字符都还留在原来的位置上。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aiohn", indices = [3,1,4,2,0]
<strong>输出:</strong> "nihao"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "aaiougrt", indices = [4,0,2,6,7,3,1,5]
<strong>输出:</strong> "arigatou"
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "art", indices = [1,0,2]
<strong>输出:</strong> "rat"
</pre>

#### 提示:
* `s.length == indices.length == n`
* `1 <= n <= 100`
* `s` 仅包含小写英文字母。
* `0 <= indices[i] < n`
* `indices` 的所有的值都是唯一的（也就是说，`indices` 是整数 `0` 到 `n - 1` 形成的一组排列）。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def restoreString(self, s: str, indices: List[int]) -> str:
        ret = [''] * len(s)

        for i in range(len(s)):
            ret[indices[i]] = s[i]

        return ''.join(ret)
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @param {Integer[]} indices
# @return {String}
def restore_string(s, indices)
    ret = [''] * s.length

    for i in 0...s.length
        ret[indices[i]] = s[i]
    end

    return ret.join()
end
```
