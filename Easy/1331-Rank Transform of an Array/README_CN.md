# 1331. 数组序号转换
给你一个整数数组 ```arr``` ，请你将数组中的每个元素替换为它们排序后的序号。

序号代表了一个元素有多大。序号编号的规则如下：
* 序号从 1 开始编号。
* 一个元素越大，那么序号越大。如果两个元素相等，那么它们的序号相同。
* 每个数字的序号都应该尽可能地小。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [40,10,20,30]
<strong>输出:</strong> [4,1,2,3]
<strong>解释:</strong> 40 是最大的元素。 10 是最小的元素。 20 是第二小的数字。 30 是第三小的数字。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [100,100,100]
<strong>输出:</strong> [1,1,1]
<strong>解释:</strong> 所有元素有相同的序号。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [37,12,28,9,100,56,80,5,12]
<strong>输出:</strong> [5,3,4,2,8,6,7,1,3]
</pre>

#### 提示:
* <code>0 <= arr.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= arr[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 排序
```Python
class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        sorted_arr = sorted(set(arr))
        rank = {n: i + 1 for i, n in enumerate(sorted_arr)}

        return [rank[n] for n in arr]
```
