# 1518. 换酒问题
小区便利店正在促销，用 `numExchange` 个空酒瓶可以兑换一瓶新酒。你购入了 `numBottles` 瓶酒。

如果喝掉了酒瓶中的酒，那么酒瓶就会变成空的。

请你计算 **最多** 能喝到多少瓶酒。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/19/sample_1_1875.png)
<pre>
<strong>输入:</strong> numBottles = 9, numExchange = 3
<strong>输出:</strong> 13
<strong>解释:</strong> 你可以用 3 个空酒瓶兑换 1 瓶酒。
所以最多能喝到 9 + 3 + 1 = 13 瓶酒。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/19/sample_2_1875.png)
<pre>
<strong>输入:</strong> numBottles = 15, numExchange = 4
<strong>输出:</strong> 19
<strong>解释:</strong> 你可以用 4 个空酒瓶兑换 1 瓶酒。
所以最多能喝到 15 + 3 + 1 = 19 瓶酒。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> numBottles = 5, numExchange = 5
<strong>输出:</strong> 6
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> numBottles = 2, numExchange = 3
<strong>输出:</strong> 2
</pre>

#### 提示:
* `1 <= numBottles <= 100`
* `2 <= numExchange <= 100`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer} num_bottles
# @param {Integer} num_exchange
# @return {Integer}
def num_water_bottles(num_bottles, num_exchange)
    ret = num_empty = num_bottles

    while num_empty >= num_exchange
        ret += num_empty / num_exchange
        num_empty = num_empty % num_exchange + num_empty / num_exchange
    end

    return ret
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_empty = num_bottles;
        let mut ret = num_bottles;

        while num_empty >= num_exchange {
            ret += num_empty / num_exchange;
            num_empty = num_empty % num_exchange + num_empty / num_exchange;
        }

        ret
    }
}
```
