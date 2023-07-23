# 551. 学生出勤记录 I
给定一个字符串来代表一个学生的出勤记录，这个记录仅包含以下三个字符：
1. **'A'** : Absent，缺勤
2. **'L'** : Late，迟到
3. **'P'** : Present，到场

如果一个学生的出勤记录中不**超过一个'A'(缺勤)**并且**不超过两个连续的'L'(迟到)**,那么这个学生会被奖赏。

你需要根据这个学生的出勤记录判断他是否会被奖赏。

#### 示例 1:
<pre>
<strong>输入:</strong> "PPALLP"
<strong>输出:</strong> True
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "PPALLL"
<strong>输出:</strong> False
</pre>

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def checkRecord(self, s: str) -> bool:
        return s.count('A') < 2 and s.count('LLL') < 1
```
