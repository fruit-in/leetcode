# 2180. 统计各位数字之和为偶数的整数个数
给你一个正整数 `num` ，请你统计并返回 **小于或等于** `num` 且各位数字之和为 **偶数** 的正整数的数目。

正整数的 **各位数字之和** 是其所有位上的对应数字相加的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 4
<strong>输出:</strong> 2
<strong>解释:</strong>
只有 2 和 4 满足小于等于 4 且各位数字之和为偶数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 30
<strong>输出:</strong> 14
<strong>解释:</strong>
只有 14 个整数满足小于等于 30 且各位数字之和为偶数，分别是：
2、4、6、8、11、13、15、17、19、20、22、24、26 和 28 。
</pre>

#### 提示:
* `1 <= num <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countEven(self, num: int) -> int:
        return sum(1 for x in range(2, num + 1)
                   if sum([x // 1000, x % 1000 // 100, x % 100 // 10, x % 10]) % 2 == 0)
```
