# 2064. Minimized Maximum of Products Distributed to Any Store
You are given an integer `n` indicating there are `n` specialty retail stores. There are `m` product types of varying amounts, which are given as a **0-indexed** integer array `quantities`, where `quantities[i]` represents the number of products of the <code>i<sup>th</sup></code> product type.

You need to distribute **all products** to the retail stores following these rules:

* A store can only be given **at most one product type** but can be given **any** amount of it.
* After distribution, each store will have been given some number of products (possibly `0`). Let `x` represent the maximum number of products given to any store. You want `x` to be as small as possible, i.e., you want to **minimize** the **maximum** number of products that are given to any store.

Return *the minimum possible* `x`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 6, quantities = [11,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> One optimal way is:
- The 11 products of type 0 are distributed to the first four stores in these amounts: 2, 3, 3, 3
- The 6 products of type 1 are distributed to the other two stores in these amounts: 3, 3
The maximum number of products given to any store is max(2, 3, 3, 3, 3, 3) = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 7, quantities = [15,10,10]
<strong>Output:</strong> 5
<strong>Explanation:</strong> One optimal way is:
- The 15 products of type 0 are distributed to the first three stores in these amounts: 5, 5, 5
- The 10 products of type 1 are distributed to the next two stores in these amounts: 5, 5
- The 10 products of type 2 are distributed to the last two stores in these amounts: 5, 5
The maximum number of products given to any store is max(5, 5, 5, 5, 5, 5, 5) = 5.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 1, quantities = [100000]
<strong>Output:</strong> 100000
<strong>Explanation:</strong> The only optimal way is:
- The 100000 products of type 0 are distributed to the only store.
The maximum number of products given to any store is max(100000) = 100000.
</pre>

#### Constraints:
* `m == quantities.length`
* <code>1 <= m <= n <= 10<sup>5</sup></code>
* <code>1 <= quantities[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut high = *quantities.iter().max().unwrap();

        while low < high {
            let x = (low + high) / 2;
            let mut y = 0;

            for q in &quantities {
                y += q / x;
                if q % x != 0 {
                    y += 1;
                }
            }

            if y > n {
                low = x + 1;
            } else {
                high = x;
            }
        }

        high
    }
}
```
