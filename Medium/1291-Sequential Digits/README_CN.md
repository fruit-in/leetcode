# 1291. 顺次数
我们定义「顺次数」为：每一位上的数字都比前一位上的数字大 `1` 的整数。

请你返回由 `[low, high]` 范围内所有顺次数组成的 **有序** 列表（从小到大排序）。

#### 示例 1:
<pre>
<strong>输入:</strong> low = 100, high = 300
<strong>输出:</strong> [123,234]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> low = 1000, high = 13000
<strong>输出:</strong> [1234,2345,3456,4567,5678,6789,12345]
</pre>

#### 提示:
* `10 <= low <= high <= 10^9`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} low
# @param {Integer} high
# @return {Integer[]}
def sequential_digits(low, high)
  ret = []

  (1..8).each do |x|
    while x <= high && x % 10 != 0
      ret.push(x) if x >= low
      x = x * 10 + x % 10 + 1
    end
  end

  ret.sort
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ret = vec![];

        for i in 1..9 {
            let mut x = i;
            while x <= high && x % 10 != 0 {
                if x >= low {
                    ret.push(x);
                }
                x = x * 10 + x % 10 + 1;
            }
        }

        ret.sort_unstable();

        ret
    }
}
```
