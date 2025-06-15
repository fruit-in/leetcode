# 1850. Minimum Adjacent Swaps to Reach the Kth Smallest Number
You are given a string `num`, representing a large integer, and an integer `k`.

We call some integer **wonderful** if it is a **permutation** of the digits in `num` and is **greater in value** than `num`. There can be many wonderful integers. However, we only care about the **smallest-valued** ones.

* For example, when `num = "5489355142"`:
    * The 1<sup>st</sup> smallest wonderful integer is `"5489355214"`.
    * The 2<sup>nd</sup> smallest wonderful integer is `"5489355241"`.
    * The 3<sup>rd</sup> smallest wonderful integer is `"5489355412"`.
    * The 4<sup>th</sup> smallest wonderful integer is `"5489355421"`.

Return *the **minimum number of adjacent digit swaps** that needs to be applied to* `num` *to reach the* <code>k<sup>th</sup></code> ***smallest wonderful** integer*.

The tests are generated in such a way that <code>k<sup>th</sup></code> smallest wonderful integer exists.

#### Example 1:
<pre>
<strong>Input:</strong> num = "5489355142", k = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> The 4th smallest wonderful number is "5489355421". To get this number:
- Swap index 7 with index 8: "5489355142" -> "5489355412"
- Swap index 8 with index 9: "5489355412" -> "5489355421"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "11112", k = 4
<strong>Output:</strong> 4
<strong>Explanation:</strong> The 4th smallest wonderful number is "21111". To get this number:
- Swap index 3 with index 4: "11112" -> "11121"
- Swap index 2 with index 3: "11121" -> "11211"
- Swap index 1 with index 2: "11211" -> "12111"
- Swap index 0 with index 1: "12111" -> "21111"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "00123", k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The 1st smallest wonderful number is "00132". To get this number:
- Swap index 3 with index 4: "00123" -> "00132"
</pre>

#### Constraints:
* `2 <= num.length <= 1000`
* `1 <= k <= 1000`
* `num` only consists of digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut num = num.into_bytes();
        let n = num.len();
        let origin = num.clone();
        let mut ret = 0;

        for _ in 0..k {
            for i in (0..n - 1).rev() {
                if num[i] >= num[i + 1] {
                    continue;
                }

                for j in (i + 1..n).rev() {
                    if num[i] >= num[j] {
                        continue;
                    }

                    num.swap(i, j);
                    for k in 0..(n - i) / 2 {
                        num.swap(i + 1 + k, n - 1 - k);
                    }
                    break;
                }
                break;
            }
        }

        for i in 0..n {
            if num[i] == origin[i] {
                continue;
            }

            for j in i + 1..n {
                if num[j] != origin[i] {
                    continue;
                }

                for k in (i + 1..=j).rev() {
                    num[k] = num[k - 1];
                }
                num[i] = origin[i];
                ret += j - i;
                break;
            }
        }

        ret as i32
    }
}
```
