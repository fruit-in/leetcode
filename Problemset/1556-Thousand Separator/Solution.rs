impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string();
        let v = s.split("").filter(|&c| c != "").collect::<Vec<_>>();
        let v = v.rchunks(3).rev().map(|c| c.concat()).collect::<Vec<_>>();

        v.join(".")
    }
}
