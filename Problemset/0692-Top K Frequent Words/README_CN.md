# 692. 前K个高频单词
给一非空的单词列表，返回前 *k* 个出现次数最多的单词。

返回的答案应该按单词出现频率由高到低排序。如果不同的单词有相同出现频率，按字母顺序排序。

#### 示例 1:
<pre>
<b>输入:</b> ["i", "love", "leetcode", "i", "love", "coding"], k = 2
<b>输出:</b> ["i", "love"]
<b>解析:</b> "i" 和 "love" 为出现次数最多的两个单词，均为2次。
    注意，按字母顺序 "i" 在 "love" 之前。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4
<b>输出:</b> ["the", "is", "sunny", "day"]
<b>解析:</b> "the", "is", "sunny" 和 "day" 是出现次数最多的四个单词，
    出现次数依次为 4, 3, 2 和 1 次。
</pre>

#### 注意:
1. 假定 *k* 总为有效值， 1 ≤ *k* ≤ 集合元素数。
2. 输入的单词均由小写字母组成。

#### 扩展练习:
1. 尝试以 *O*(*n* log *k*) 时间复杂度和 *O*(*n*) 空间复杂度解决。

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {String[]} words
# @param {Integer} k
# @return {String[]}
def top_k_frequent(words, k)
    count = Hash.new(0)

    for word in words
        count[word] += 1
    end

    return count.keys.sort_by {|k| [-count[k], k]}[0...k]
end
```
