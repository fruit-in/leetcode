# 927. 三等分
给定一个由 `0` 和 `1` 组成的数组 `arr` ，将数组分成  **3 个非空的部分** ，使得所有这些部分表示相同的二进制值。

如果可以做到，请返回**任何** `[i, j]`，其中 `i+1 < j`，这样一来：

* `arr[0], arr[1], ..., arr[i]` 为第一部分；
* `arr[i + 1], arr[i + 2], ..., arr[j - 1]` 为第二部分；
* `arr[j], arr[j + 1], ..., arr[arr.length - 1]` 为第三部分。
* 这三个部分所表示的二进制值相等。

如果无法做到，就返回 `[-1, -1]`。

注意，在考虑每个部分所表示的二进制时，应当将其看作一个整体。例如，`[1,1,0]` 表示十进制中的 `6`，而不会是 `3`。此外，前导零也是**被允许**的，所以 `[0,1,1]` 和 `[1,1]` 表示相同的值。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,0,1,0,1]
<strong>输出:</strong> [0,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,0,1,1]
<strong>输出:</strong> [-1,-1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,1,0,0,1]
<strong>输出:</strong> [0,2]
</pre>

#### 提示:
* <code>3 <= arr.length <= 3 * 10<sup>4</sup></code>
* `arr[i]` 是 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let ones = arr.iter().filter(|&&x| x == 1).count() as i32;

        if ones % 3 != 0 {
            return vec![-1, -1];
        } else if ones == 0 {
            return vec![0, 2];
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = arr.len() - 1;
        let mut count = 0;

        while arr[k] == 0 {
            k -= 1;
        }

        while count < ones / 3 {
            count += arr[i];
            i += 1;
        }
        for _ in k..arr.len() - 1 {
            if arr[i] == 1 {
                return vec![-1, -1];
            }

            i += 1;
        }
        i -= 1;

        j = i + 1;
        count = 0;
        while count < ones / 3 {
            count += arr[j];
            j += 1;
        }
        for _ in k..arr.len() - 1 {
            if arr[j] == 1 {
                return vec![-1, -1];
            }

            j += 1;
        }

        for k in 0..(i + 1).min(arr.len() - j).min(j - i - 1) {
            let (a, b, c) = (arr[i - k], arr[j - 1 - k], arr[arr.len() - 1 - k]);
            if a != b || b != c || a != c {
                return vec![-1, -1];
            }
        }

        vec![i as i32, j as i32]
    }
}
```
