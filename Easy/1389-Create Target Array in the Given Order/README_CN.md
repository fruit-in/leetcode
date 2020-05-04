# 1389. 按既定顺序创建目标数组
给你两个整数数组 ```nums``` 和 ```index```。你需要按照以下规则创建目标数组：
* 目标数组 ```target``` 最初为空。
* 按从左到右的顺序依次读取 ```nums[i]``` 和 ```index[i]```，在 ```target``` 数组中的下标 ```index[i]``` 处插入值 ```nums[i]``` 。
* 重复上一步，直到在 ```nums``` 和 ```index``` 中都没有要读取的元素。

请你返回目标数组。

题目保证数字插入位置总是存在。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,2,3,4], index = [0,1,2,2,1]
<strong>输出:</strong> [0,4,1,3,2]
<strong>解释:</strong>
nums       index     target
0            0        [0]
1            1        [0,1]
2            2        [0,1,2]
3            2        [0,1,3,2]
4            1        [0,4,1,3,2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,0], index = [0,1,2,3,0]
<strong>输出:</strong> [0,1,2,3,4]
<strong>解释:</strong>
nums       index     target
1            0        [1]
2            1        [1,2]
3            2        [1,2,3]
4            3        [1,2,3,4]
0            0        [0,1,2,3,4]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1], index = [0]
<strong>输出:</strong> [1]
</pre>

#### 提示:
* ```1 <= nums.length, index.length <= 100```
* ```nums.length == index.length```
* ```0 <= nums[i] <= 100```
* ```0 <= index[i] <= i```

## 题解 (Rust)

### 1. 模拟
```Rust
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut target = Vec::new();

        for i in 0..nums.len() {
            target.insert(index[i] as usize, nums[i]);
        }

        target
    }
}
```
