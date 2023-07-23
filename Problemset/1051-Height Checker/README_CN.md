# 1051. 高度检查器
学校在拍年度纪念照时，一般要求学生按照 **非递减** 的高度顺序排列。

请你返回至少有多少个学生没有站在正确位置数量。该人数指的是：能让所有学生以 **非递减** 高度排列的必要移动人数。

#### 示例:
<pre>
<strong>输入:</strong> [1,1,4,2,1,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 
高度为 4、3 和最后一个 1 的学生，没有站在正确的位置。
</pre>

#### 提示:
1. ```1 <= heights.length <= 100```
2. ```1 <= heights[i] <= 100```

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def heightChecker(self, heights: List[int]) -> int:
        return len(list(filter(lambda x : x[0] != x[1], zip(sorted(heights), heights))))
```
