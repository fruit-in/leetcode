# 1089. 复写零
给你一个长度固定的整数数组 ```arr```，请你将该数组中出现的每个零都复写一遍，并将其余的元素向右平移。

注意：请不要在超过该数组长度的位置写入元素。

要求：请对输入的数组 **就地** 进行上述修改，不要从函数返回任何东西。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,0,2,3,0,4,5,0]
<strong>输出:</strong> null
<strong>解释:</strong> 调用函数后，<strong>输入</strong>的数组将被修改为：[1,0,0,2,3,0,0,4]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3]
<strong>输出:</strong> null
<strong>解释:</strong> 调用函数后，<strong>输入</strong>的数组将被修改为：[1,2,3]
</pre>

#### 提示:
1. ```1 <= arr.length <= 10000```
2. ```0 <= arr[i] <= 9```

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                for j in ((i + 1)..arr.len()).rev() {
                    arr[j] = arr[j - 1];
                }
                i += 1;
            }
            i += 1;
        }
    }
}
```

### 2. 对0计数
```Rust
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut zeroes = 0;

        for i in 0..arr.len() {
            if i + zeroes / 2 >= arr.len() - 1 {
                if arr[i] == 0 && i + zeroes / 2 == arr.len() - 1 {
                    zeroes += 1;
                }
                break;
            }

            if arr[i] == 0 {
                zeroes += 2;
            }
        }

        for i in (0..arr.len()).rev() {
            arr[i] = arr[i - zeroes / 2];
            if arr[i] == 0 {
                zeroes -= 1;
            }
        }
    }
}
```
