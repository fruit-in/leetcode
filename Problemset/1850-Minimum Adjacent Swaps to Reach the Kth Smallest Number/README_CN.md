# 1850. 邻位交换的最小次数
给你一个表示大整数的字符串 `num` ，和一个整数 `k` 。

如果某个整数是 `num` 中各位数字的一个 **排列** 且它的 **值大于** `num` ，则称这个整数为 **妙数** 。可能存在很多妙数，但是只需要关注 **值最小** 的那些。

* 例如，`num = "5489355142"` ：
    * 第 1 个最小妙数是 `"5489355214"`
    * 第 2 个最小妙数是 `"5489355241"`
    * 第 3 个最小妙数是 `"5489355412"`
    * 第 4 个最小妙数是 `"5489355421"`

返回要得到第 `k` 个 **最小妙数** 需要对 `num` 执行的 **相邻位数字交换的最小次数** 。

测试用例是按存在第 `k` 个最小妙数而生成的。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "5489355142", k = 4
<strong>输出:</strong> 2
<strong>解释:</strong> 第 4 个最小妙数是 "5489355421" ，要想得到这个数字：
- 交换下标 7 和下标 8 对应的位："5489355142" -> "5489355412"
- 交换下标 8 和下标 9 对应的位："5489355412" -> "5489355421"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "11112", k = 4
<strong>输出:</strong> 4
<strong>解释:</strong> 第 4 个最小妙数是 "21111" ，要想得到这个数字：
- 交换下标 3 和下标 4 对应的位："11112" -> "11121"
- 交换下标 2 和下标 3 对应的位："11121" -> "11211"
- 交换下标 1 和下标 2 对应的位："11211" -> "12111"
- 交换下标 0 和下标 1 对应的位："12111" -> "21111"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = "00123", k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 第 1 个最小妙数是 "00132" ，要想得到这个数字：
- 交换下标 3 和下标 4 对应的位："00123" -> "00132"
</pre>

#### 提示:
* `2 <= num.length <= 1000`
* `1 <= k <= 1000`
* `num` 仅由数字组成

## 题解 (Rust)

### 1. 题解
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
