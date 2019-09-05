impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut even = Vec::new();
        let mut odd = Vec::new();
        for n in a {
            match n % 2 == 0 {
                true => even.push(n),
                false => odd.push(n),
            };
        }
        even.append(&mut odd);
        even
    }
}
