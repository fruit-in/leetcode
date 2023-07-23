impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let v: Vec<i32> = date.split('-').map(|s| s.parse().unwrap()).collect();
        let mut m = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
        let mut ans = v[2];
        if (v[0] % 4 == 0 && v[0] % 100 != 0) || v[0] % 400 == 0 {
            m[1] += 1;
        }
        for i in 1..v[1] {
            ans += m[i as usize - 1];
        }
        ans
    }
}
