# 888. 公平的糖果交换
爱丽丝和鲍勃有不同大小的糖果棒：```A[i]``` 是爱丽丝拥有的第 ```i``` 块糖的大小，```B[j]``` 是鲍勃拥有的第 ```j``` 块糖的大小。

因为他们是朋友，所以他们想交换一个糖果棒，这样交换后，他们都有相同的糖果总量。*（一个人拥有的糖果总量是他们拥有的糖果棒大小的总和。）*

返回一个整数数组 ```ans```，其中 ```ans[0]``` 是爱丽丝必须交换的糖果棒的大小，```ans[1]``` 是 Bob 必须交换的糖果棒的大小。

如果有多个答案，你可以返回其中任何一个。保证答案存在。

#### 示例 1:
<pre>
<strong>输入:</strong> A = [1,1], B = [2,2]
<strong>输出:</strong> [1,2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = [1,2], B = [2,3]
<strong>输出:</strong> [1,2]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> A = [2], B = [1,3]
<strong>输出:</strong> [2,3]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> A = [1,2,5], B = [2,4]
<strong>输出:</strong> [5,4]
</pre>

#### 提示:
* ```1 <= A.length <= 10000```
* ```1 <= B.length <= 10000```
* ```1 <= A[i] <= 100000```
* ```1 <= B[i] <= 100000```
* 保证爱丽丝与鲍勃的糖果总量不同。
* 答案肯定存在。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let sum_a = a.iter().sum::<i32>();
        let sum_b = b.iter().sum::<i32>();

        for exchange_a in a {
            let exchange_b = (sum_b - sum_a) / 2 + exchange_a;
            if b.contains(&exchange_b) {
                return vec![exchange_a, exchange_b];
            }
        }

        Vec::new()
    }
}
```
