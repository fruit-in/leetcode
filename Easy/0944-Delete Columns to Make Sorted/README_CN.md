# 944. 删列造序
给定由 ```N``` 个小写字母字符串组成的数组 ```A```，其中每个字符串长度相等。

**删除** 操作的定义是：选出一组要删掉的列，删去 ```A``` 中对应列中的所有字符，形式上，第 ```n``` 列为 ```[A[0][n], A[1][n], ..., A[A.length-1][n]]```）。

比如，有 ```A = ["abcdef", "uvwxyz"]```，
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/07/06/944_1.png)

要删掉的列为 ```{0, 2, 3}```，删除后 ```A``` 为```["bef", "vyz"]```， ```A``` 的列分别为```["b","v"], ["e","y"], ["f","z"]```。
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/07/06/944_2.png)

你需要选出一组要删掉的列 ```D```，对 ```A``` 执行删除操作，使 ```A``` 中剩余的每一列都是 **非降序** 排列的，然后请你返回 ```D.length``` 的最小可能值。

#### 示例 1:
<pre>
<strong>输入:</strong> ["cba","daf","ghi"]
<strong>输出:</strong> 1
<strong>解释:</strong>
当选择 D = {1}，删除后 A 的列为：["c","d","g"] 和 ["a","f","i"]，均为非降序排列。
若选择 D = {}，那么 A 的列 ["b","a","h"] 就不是非降序排列了。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["a","b"]
<strong>输出:</strong> 0
<strong>解释:</strong> D = {}
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> ["zyx","wvu","tsr"]
<strong>输出:</strong> 3
<strong>解释:</strong> D = {0, 1, 2}
</pre>

#### 提示:
1. ```1 <= A.length <= 100```
2. ```1 <= A[i].length <= 1000```

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def minDeletionSize(self, A: List[str]) -> int:
        return len(list(filter(lambda B : B != sorted(B), map(list, zip(*A)))))
```
