# 2241. Design an ATM Machine
There is an ATM machine that stores banknotes of `5` denominations: `20`, `50`, `100`, `200`, and `500` dollars. Initially the ATM is empty. The user can use the machine to deposit or withdraw any amount of money.

When withdrawing, the machine prioritizes using banknotes of **larger** values.

* For example, if you want to withdraw `$300` and there are `2` `$50` banknotes, `1` `$100` banknote, and `1` `$200` banknote, then the machine will use the `$100` and `$200` banknotes.
* However, if you try to withdraw `$600` and there are `3` `$200` banknotes and `1` `$500` banknote, then the withdraw request will be rejected because the machine will first try to use the `$500` banknote and then be unable to use banknotes to complete the remaining `$100`. Note that the machine is **not** allowed to use the `$200` banknotes instead of the `$500` banknote.

Implement the ATM class:

* `ATM()` Initializes the ATM object.
* `void deposit(int[] banknotesCount)` Deposits new banknotes in the order `$20`, `$50`, `$100`, `$200`, and `$500`.
* `int[] withdraw(int amount)` Returns an array of length `5` of the number of banknotes that will be handed to the user in the order `$20`, `$50`, `$100`, `$200`, and `$500`, and update the number of banknotes in the ATM after withdrawing. Returns `[-1]` if it is not possible (do **not** withdraw any banknotes in this case).

#### Example 1:
<pre>
<strong>Input:</strong>
["ATM", "deposit", "withdraw", "deposit", "withdraw", "withdraw"]
[[], [[0,0,1,2,1]], [600], [[0,1,0,1,1]], [600], [550]]
<strong>Output:</strong>
[null, null, [0,0,1,0,1], null, [-1], [0,1,0,0,1]]
<strong>Explanation:</strong>
ATM atm = new ATM();
atm.deposit([0,0,1,2,1]); // Deposits 1 $100 banknote, 2 $200 banknotes,
                          // and 1 $500 banknote.
atm.withdraw(600);        // Returns [0,0,1,0,1]. The machine uses 1 $100 banknote
                          // and 1 $500 banknote. The banknotes left over in the
                          // machine are [0,0,0,2,0].
atm.deposit([0,1,0,1,1]); // Deposits 1 $50, $200, and $500 banknote.
                          // The banknotes in the machine are now [0,1,0,3,1].
atm.withdraw(600);        // Returns [-1]. The machine will try to use a $500 banknote
                          // and then be unable to complete the remaining $100,
                          // so the withdraw request will be rejected.
                          // Since the request is rejected, the number of banknotes
                          // in the machine is not modified.
atm.withdraw(550);        // Returns [0,1,0,0,1]. The machine uses 1 $50 banknote
                          // and 1 $500 banknote.
</pre>

#### Constraints:
* `banknotesCount.length == 5`
* <code>0 <= banknotesCount[i] <= 10<sup>9</sup></code>
* <code>1 <= amount <= 10<sup>9</sup></code>
* At most `5000` calls **in total** will be made to `withdraw` and `deposit`.
* At least **one** call will be made to each function `withdraw` and `deposit`.

## Solutions (Rust)

### 1. Solution
```Rust
struct ATM {
    banknotes_count: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        Self {
            banknotes_count: vec![0; 5],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..5 {
            self.banknotes_count[i] += banknotes_count[i] as i64;
        }
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let banknotes = [20, 50, 100, 200, 500];
        let mut amount = amount as i64;
        let mut ret = vec![0; 5];

        for i in (0..5).rev() {
            ret[i] += (amount / banknotes[i]).min(self.banknotes_count[i]);
            amount -= ret[i] * banknotes[i];
        }

        if amount > 0 {
            return vec![-1];
        }

        for i in 0..5 {
            self.banknotes_count[i] -= ret[i];
        }

        ret.into_iter().map(|x| x as i32).collect()
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */
```
