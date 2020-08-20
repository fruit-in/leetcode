# 477. 汉明距离总和
两个整数的 [汉明距离](https://baike.baidu.com/item/%E6%B1%89%E6%98%8E%E8%B7%9D%E7%A6%BB/475174?fr=aladdin) 指的是这两个数字的二进制数对应位不同的数量。

计算一个数组中，任意两个数之间汉明距离的总和。

#### 示例:
<pre>
<strong>输入:</strong> 4, 14, 2
<strong>输出:</strong> 6
<strong>解释:</strong> 在二进制表示中，4表示为0100，14表示为1110，2表示为0010。（这样表示是为了体现后四位之间关系）
所以答案为：
HammingDistance(4, 14) + HammingDistance(4, 2) + HammingDistance(14, 2) = 2 + 2 + 2 = 6.
</pre>

#### 注意:
1. 数组中元素的范围为从 `0`到 `10^9`。
2. 数组的长度不超过 `10^4`。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def total_hamming_distance(nums)
    ret = 0

    for i in 0...30
        zeros, ones = 0, 0
        for num in nums
            if (1 << i) & num == 0
                zeros += 1
            else
                ones += 1
            end
        end
        ret += zeros * ones
    end

    return ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 0..30 {
            let mut zeros = 0;
            let mut ones = 0;
            for num in &nums {
                match (1 << i) & num {
                    0 => zeros += 1,
                    _ => ones += 1,
                }
            }
            ret += zeros * ones;
        }

        ret
    }
}
```
