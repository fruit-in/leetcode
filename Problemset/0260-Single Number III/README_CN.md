# 260. 只出现一次的数字 III
给定一个整数数组 ```nums```，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。

#### 示例:
<pre>
<strong>输入:</strong> [1,2,1,3,2,5]
<strong>输出:</strong> [3,5]
</pre>

#### 注意:
1. 结果输出的顺序并不重要，对于上面的例子， ```[5, 3]``` 也是正确答案。
2. 你的算法应该具有线性时间复杂度。你能否仅使用常数空间复杂度来实现？

## 题解 (Rust)

### 1. 异或
```Rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let acc = nums.iter().fold(0, |acc, n| acc ^ n);
        let mask = acc & (-acc);
        let mut ret = vec![0, 0];

        for n in nums {
            match n & mask {
                0 => ret[0] ^= n,
                _ => ret[1] ^= n,
            }
        }

        ret
    }
}
```
