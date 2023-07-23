# 1460. 通过翻转子数组使两个数组相等
给你两个长度相同的整数数组 `target` 和 `arr` 。

每一步中，你可以选择 `arr` 的任意 **非空子数组** 并将它翻转。你可以执行此过程任意次。

如果你能让 `arr` 变得与 `target` 相同，返回 True；否则，返回 False 。

#### 示例 1:
<pre>
<strong>输入:</strong> target = [1,2,3,4], arr = [2,4,1,3]
<strong>输出:</strong> true
<strong>解释:</strong> 你可以按照如下步骤使 arr 变成 target：
1- 翻转子数组 [2,4,1] ，arr 变成 [1,4,2,3]
2- 翻转子数组 [4,2] ，arr 变成 [1,2,4,3]
3- 翻转子数组 [4,3] ，arr 变成 [1,2,3,4]
上述方法并不是唯一的，还存在多种将 arr 变成 target 的方法。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = [7], arr = [7]
<strong>输出:</strong> true
<strong>解释:</strong> arr 不需要做任何翻转已经与 target 相等。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> target = [1,12], arr = [12,1]
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> target = [3,7,9], arr = [3,7,11]
<strong>输出:</strong> false
<strong>解释:</strong> arr 没有数字 9 ，所以无论如何也无法变成 target 。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> target = [1,1,1,1,1], arr = [1,1,1,1,1]
<strong>输出:</strong> true
</pre>

#### 提示:
* `target.length == arr.length`
* `1 <= target.length <= 1000`
* `1 <= target[i] <= 1000`
* `1 <= arr[i] <= 1000`

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target = target;
        let mut arr = arr;
        target.sort_unstable();
        arr.sort_unstable();

        target == arr
    }
}
```
