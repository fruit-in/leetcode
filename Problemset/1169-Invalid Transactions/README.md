# 1169. Invalid Transactions
A transaction is possibly invalid if:

* the amount exceeds `$1000`, or;
* if it occurs within (and including) `60` minutes of another transaction with the **same name** in a **different city**.

You are given an array of strings `transaction` where `transactions[i]` consists of comma-separated values representing the name, time (in minutes), amount, and city of the transaction.

Return a list of `transactions` that are possibly invalid. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> transactions = ["alice,20,800,mtv","alice,50,100,beijing"]
<strong>Output:</strong> ["alice,20,800,mtv","alice,50,100,beijing"]
<strong>Explanation:</strong> The first transaction is invalid because the second transaction occurs within a difference of 60 minutes, have the same name and is in a different city. Similarly the second one is invalid too.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> transactions = ["alice,20,800,mtv","alice,50,1200,mtv"]
<strong>Output:</strong> ["alice,50,1200,mtv"]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> transactions = ["alice,20,800,mtv","bob,50,1200,mtv"]
<strong>Output:</strong> ["bob,50,1200,mtv"]
</pre>

#### Constraints:
* `transactions.length <= 1000`
* Each `transactions[i]` takes the form `"{name},{time},{amount},{city}"`
* Each `{name}` and `{city}` consist of lowercase English letters, and have lengths between `1` and `10`.
* Each `{time}` consist of digits, and represent an integer between `0` and `1000`.
* Each `{amount}` consist of digits, and represent an integer between `0` and `2000`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut ret = vec![];

        for i in 0..transactions.len() {
            let transaction0 = transactions[i].split(',').collect::<Vec<_>>();
            let name0 = transaction0[0];
            let time0 = transaction0[1].parse::<i32>().unwrap();
            let amount0 = transaction0[2].parse::<i32>().unwrap();
            let city0 = transaction0[3];

            if amount0 > 1000 {
                ret.push(transactions[i].clone());
                continue;
            }

            for j in 0..transactions.len() {
                if j != i {
                    let transaction1 = transactions[j].split(',').collect::<Vec<_>>();
                    let name1 = transaction1[0];
                    let time1 = transaction1[1].parse::<i32>().unwrap();
                    let amount1 = transaction1[2].parse::<i32>().unwrap();
                    let city1 = transaction1[3];

                    if name0 == name1 && (time0 - time1).abs() <= 60 && city0 != city1 {
                        ret.push(transactions[i].clone());
                        break;
                    }
                }
            }
        }

        ret
    }
}
```
