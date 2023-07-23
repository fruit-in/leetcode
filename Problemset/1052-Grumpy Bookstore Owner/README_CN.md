# 1052. 爱生气的书店老板
今天，书店老板有一家店打算试营业 ```customers.length``` 分钟。每分钟都有一些顾客（```customers[i]```）会进入书店，所有这些顾客都会在那一分钟结束后离开。

在某些时候，书店老板会生气。 如果书店老板在第 ```i``` 分钟生气，那么 ```grumpy[i] = 1```，否则 ```grumpy[i] = 0```。 当书店老板生气时，那一分钟的顾客就会不满意，不生气则他们是满意的。

书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 ```X``` 分钟不生气，但却只能使用一次。

请你返回这一天营业下来，最多有多少客户能够感到满意的数量。

#### 示例:
<pre>
<strong>Input:</strong> customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
<strong>Output:</strong> 16
<strong>Explanation:</strong> 书店老板在最后 3 分钟保持冷静。
感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.
</pre>

#### 提示:
* ```1 <= X <= customers.length == grumpy.length <= 20000```
* ```0 <= customers[i] <= 1000```
* ```0 <= grumpy[i] <= 1```

## 题解 (Ruby)

### 1. 滑动窗口
```Ruby
# @param {Integer[]} customers
# @param {Integer[]} grumpy
# @param {Integer} x
# @return {Integer}
def max_satisfied(customers, grumpy, x)
    curr = 0
    ret = 0

    for i in 0...grumpy.length
        if i < x or grumpy[i] == 0
            curr += customers[i]
            ret += customers[i]
        end
        curr -= customers[i - x] if i >= x and grumpy[i - x] == 1
        curr += customers[i] if i >= x and grumpy[i] == 1

        ret = [ret, curr].max
    end

    return ret
end
```

## 题解 (Rust)

### 1. 滑动窗口
```Rust
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut curr = 0;
        let mut ret = 0;

        for i in 0..grumpy.len() {
            if i < x as usize || grumpy[i] == 0 {
                curr += customers[i];
                ret += customers[i];
            }
            if i >= x as usize && grumpy[i - x as usize] == 1 {
                curr -= customers[i - x as usize];
            }
            if i >= x as usize && grumpy[i] == 1 {
                curr += customers[i];
            }

            ret = ret.max(curr);
        }

        ret
    }
}
```
