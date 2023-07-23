# 1299. 将每个元素替换为右侧最大元素
给你一个数组 ```arr``` ，请你将每个元素用它右边最大的元素替换，如果是最后一个元素，用 ```-1``` 替换。

完成所有替换操作后，请你返回这个数组。

#### 示例:
<pre>
<strong>输入:</strong> arr = [17,18,5,4,6,1]
<strong>输出:</strong> [18,6,6,6,1,-1]
</pre>

#### 提示:
* ```1 <= arr.length <= 10^4```
* ```1 <= arr[i] <= 10^5```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut greatest = -1;

        for i in (0..arr.len()).rev() {
            let tmp = greatest;
            greatest = greatest.max(arr[i]);
            arr[i] = tmp;
        }

        arr
    }
}
```
