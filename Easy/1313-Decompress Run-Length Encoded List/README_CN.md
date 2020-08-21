# 1313. 解压缩编码列表
给你一个以行程长度编码压缩的整数列表 ```nums``` 。

考虑每相邻两个元素 ```[a, b] = [nums[2*i], nums[2*i+1]]``` （其中 ```i >= 0``` ），每一对都表示解压后有 ```a``` 个值为 ```b``` 的元素。

请你返回解压后的列表。

#### 示例:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> [2,4,4,4]
</pre>

#### 提示:
* ```2 <= nums.length <= 100```
* ```nums.length % 2 == 0```
* ```1 <= nums[i] <= 100```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer[]}
def decompress_rl_elist(nums)
    ret = []

    for i in (0...nums.length).step(2)
        ret.concat([nums[i + 1]] * nums[i])
    end

    return ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        for i in (0..nums.len()).step_by(2) {
            ret.append(&mut vec![nums[i + 1]; nums[i] as usize]);
        }

        ret
    }
}
```
