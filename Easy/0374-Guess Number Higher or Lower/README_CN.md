# 374. 猜数字大小
我们正在玩一个猜数字游戏。 游戏规则如下：
我从 **1** 到 ***n*** 选择一个数字。 你需要猜我选择了哪个数字。
每次你猜错了，我会告诉你这个数字是大了还是小了。
你调用一个预先定义好的接口 ```guess(int num)```，它会返回 3 个可能的结果（```-1```，```1``` 或 ```0```）：
```
-1 : 我的数字比较小
 1 : 我的数字比较大
 0 : 恭喜！你猜对了！
```

#### 示例:
<pre>
<strong>输入:</strong> n = 10, pick = 6
<strong>输出:</strong> 6
</pre>

## 题解 (Python)

### 1. 二分查找
```Python
# The guess API is already defined for you.
# @param num, your guess
# @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
# def guess(num):

class Solution(object):
    def guessNumber(self, n):
        """
        :type n: int
        :rtype: int
        """
        l, r = 1, n
        while l <= r:
            m = (l + r) // 2
            res = guess(m)
            if res == 0:
                return m
            elif res == 1:
                l = m + 1
            elif res == -1:
                r = m - 1
```
