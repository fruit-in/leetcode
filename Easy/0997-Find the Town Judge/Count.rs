impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut people = vec![0; n as usize];

        for t in trust {
            people[t[0] as usize - 1] -= 1;
            people[t[1] as usize - 1] += 1;
        }

        for i in 0..n {
            if people[i as usize] == n - 1 {
                return i + 1;
            }
        }

        -1
    }
}
