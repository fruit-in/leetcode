# 1005. Maximize Sum Of Array After K Negations
Given an array ```A``` of integers, we **must** modify the array in the following way: we choose an ```i``` and replace ```A[i]``` with ```-A[i]```, and we repeat this process ```K``` times in total.  (We may choose the same index ```i``` multiple times.)

Return the largest possible sum of the array after modifying it in this way.

#### Example 1:
<pre>
<strong>Input:</strong> A = [4,2,3], K = 1
<strong>Output:</strong> 5
<strong>Explanation:</strong> Choose indices (1,) and A becomes [4,-2,3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = [3,-1,0,2], K = 3
<strong>Output:</strong> 6
<strong>Explanation:</strong> Choose indices (1, 2, 2) and A becomes [3,1,0,2].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> A = [2,-3,-1,5,-4], K = 2
<strong>Output:</strong> 13
<strong>Explanation:</strong> Choose indices (1, 4) and A becomes [2,3,-1,5,4].
</pre>

#### Note:
1. ```1 <= A.length <= 10000```
2. ```1 <= K <= 10000```
3. ```-100 <= A[i] <= 100```

## Solutions (Rust)

### 1. Sort
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

### 2. Store Negative Numbers
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
