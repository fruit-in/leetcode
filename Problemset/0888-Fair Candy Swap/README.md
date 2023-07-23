# 888. Fair Candy Swap
Alice and Bob have candy bars of different sizes: ```A[i]``` is the size of the ```i```-th bar of candy that Alice has, and ```B[j]``` is the size of the ```j```-th bar of candy that Bob has.

Since they are friends, they would like to exchange one candy bar each so that after the exchange, they both have the same total amount of candy.  (*The total amount of candy a person has is the sum of the sizes of candy bars they have.*)

Return an integer array ```ans``` where ```ans[0]``` is the size of the candy bar that Alice must exchange, and ```ans[1]``` is the size of the candy bar that Bob must exchange.

If there are multiple answers, you may return any one of them.  It is guaranteed an answer exists.

#### Example 1:
<pre>
<strong>Input:</strong> A = [1,1], B = [2,2]
<strong>Output:</strong> [1,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = [1,2], B = [2,3]
<strong>Output:</strong> [1,2]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> A = [2], B = [1,3]
<strong>Output:</strong> [2,3]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> A = [1,2,5], B = [2,4]
<strong>Output:</strong> [5,4]
</pre>

#### Note:
* ```1 <= A.length <= 10000```
* ```1 <= B.length <= 10000```
* ```1 <= A[i] <= 100000```
* ```1 <= B[i] <= 100000```
* It is guaranteed that Alice and Bob have different total amounts of candy.
* It is guaranteed there exists an answer.

## Solutions (Rust)

### 1. Solution
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
