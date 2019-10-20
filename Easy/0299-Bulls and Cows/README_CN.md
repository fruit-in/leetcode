# 299. 猜数字游戏
你正在和你的朋友玩 [猜数字（Bulls and Cows）](https://baike.baidu.com/item/%E7%8C%9C%E6%95%B0%E5%AD%97/83200?fromtitle=Bulls+and+Cows&fromid=12003488&fr=aladdin)游戏：你写下一个数字让你的朋友猜。每次他猜测后，你给他一个提示，告诉他有多少位数字和确切位置都猜对了（称为“Bulls”, 公牛），有多少位数字猜对了但是位置不对（称为“Cows”, 奶牛）。你的朋友将会根据提示继续猜，直到猜出秘密数字。

请写出一个根据秘密数字和朋友的猜测数返回提示的函数，用 ```A``` 表示公牛，用 ```B``` 表示奶牛。

请注意秘密数字和朋友的猜测数都可能含有重复数字。

#### 示例 1:
<pre>
<strong>输入:</strong> secret = "1807", guess = "7810"
<strong>输出:</strong> "1A3B"
<strong>解释:</strong> 1 公牛和 3 奶牛。公牛是 8，奶牛是 0, 1 和 7。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> secret = "1123", guess = "0111"
<strong>输出:</strong> "1A1B"
<strong>解释:</strong> 朋友猜测数中的第一个 1 是公牛，第二个或第三个 1 可被视为奶牛。
</pre>

**说明:** 你可以假设秘密数字和朋友的猜测数都只包含数字，并且它们的长度永远相等。

## 题解 (Python)

### 1. 一次遍历
```Python3
class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        bulls = 0

        scows = [0] * 10
        gcows = [0] * 10

        for sch, gch in zip(secret, guess):
            if sch == gch:
                bulls += 1
            else:
                scows[int(sch)] += 1
                gcows[int(gch)] += 1

        cows = sum(min(scow, gcow) for scow, gcow in zip(scows, gcows))

        return "%dA%dB" % (bulls, cows)
```
