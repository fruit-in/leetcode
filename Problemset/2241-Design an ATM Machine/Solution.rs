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
