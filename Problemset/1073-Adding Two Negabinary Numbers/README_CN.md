# 1073. 负二进制数相加
给出基数为 **-2** 的两个数 `arr1` 和 `arr2`，返回两数相加的结果。

数字以 *数组形式* 给出：数组由若干 0 和 1 组成，按最高有效位到最低有效位的顺序排列。例如，`arr = [1,1,0,1]` 表示数字 `(-2)^3 + (-2)^2 + (-2)^0 = -3`。*数组形式* 中的数字 `arr` 也同样不含前导零：即 `arr == [0]` 或 `arr[0] == 1`。

返回相同表示形式的 `arr1` 和 `arr2` 相加的结果。两数的表示形式为：不含前导零、由若干 0 和 1 组成的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> arr1 = [1,1,1,1,1], arr2 = [1,0,1]
<strong>输出:</strong> [1,0,0,0,0]
<strong>解释:</strong> arr1 表示 11，arr2 表示 5，输出表示 16 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr1 = [0], arr2 = [0]
<strong>输出:</strong> [0]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr1 = [0], arr2 = [1]
<strong>输出:</strong> [1]
</pre>

#### 提示:
* `1 <= arr1.length, arr2.length <= 1000`
* `arr1[i]` 和 `arr2[i]` 都是 `0` 或 `1`
* `arr1` 和 `arr2` 都没有前导0

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut arr2 = arr2;
        let mut carry = 0;
        let mut ret = vec![];

        while !arr1.is_empty() || !arr2.is_empty() || carry != 0 {
            let x = arr1.pop().unwrap_or(0) + arr2.pop().unwrap_or(0) + carry;
            carry = 1 - (x + 2) / 2;
            ret.push(x.rem_euclid(2));
        }

        while ret.len() > 1 && *ret.last().unwrap() == 0 {
            ret.pop();
        }

        ret.reverse();

        ret
    }
}
```
