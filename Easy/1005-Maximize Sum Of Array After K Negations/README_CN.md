# 1005. K 次取反后最大化的数组和
给定一个整数数组 A，我们**只能**用以下方法修改该数组：我们选择某个个索引 ```i``` 并将 ```A[i]``` 替换为 ```-A[i]```，然后总共重复这个过程 ```K``` 次。（我们可以多次选择同一个索引 ```i```。）

以这种方式修改数组后，返回数组可能的最大和。

#### 示例 1:
<pre>
<strong>输入:</strong> A = [4,2,3], K = 1
<strong>输出:</strong> 5
<strong>解释:</strong> 选择索引 (1,) ，然后 A 变为 [4,-2,3]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = [3,-1,0,2], K = 3
<strong>输出:</strong> 6
<strong>解释:</strong> 选择索引 (1, 2, 2) ，然后 A 变为 [3,1,0,2]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> A = [2,-3,-1,5,-4], K = 2
<strong>输出:</strong> 13
<strong>解释:</strong> 选择索引 (1, 4) ，然后 A 变为 [2,3,-1,5,4]。
</pre>

#### 提示:
1. ```1 <= A.length <= 10000```
2. ```1 <= K <= 10000```
3. ```-100 <= A[i] <= 100```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut a = a;
        a.sort_unstable();

        for i in 0..a.len() {
            if a[i] < 0 && k > 0 {
                a[i] = -a[i];
                k -= 1;
            } else if k % 2 == 0 {
                break;
            } else if i > 0 && a[i] > a[i - 1] {
                a[i - 1] = -a[i - 1];
                break;
            } else {
                a[i] = -a[i];
                break;
            }
        }

        a.iter().sum()
    }
}
```

### 2. 存储负数
```Rust
impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut cnt_neg = [0; 101];
        let mut sum = 0;
        let mut min_abs = 100;

        for n in a {
            if n >= 0 {
                sum += n;
            } else {
                cnt_neg[-n as usize] += 1;
            }
            min_abs = min_abs.min(n.abs());
        }

        for i in (1..101).rev() {
            if k > 0 {
                if cnt_neg[i as usize] <= k {
                    sum += i * cnt_neg[i as usize];
                    k -= cnt_neg[i as usize];
                } else {
                    sum += i * (2 * k - cnt_neg[i as usize]);
                    k = 0;
                }
            } else {
                sum += -i * cnt_neg[i as usize];
            }
        }

        if k % 2 == 1 {
            sum -= 2 * min_abs;
        }

        sum
    }
}
```
