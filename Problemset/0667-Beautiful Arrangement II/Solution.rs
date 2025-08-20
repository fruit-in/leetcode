impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut answer = vec![];

        for x in 1..=k / 2 {
            answer.push(x);
            answer.push(n + 1 - x);
        }

        if k % 2 == 1 {
            for x in k / 2 + 1..=n - k / 2 {
                answer.push(x);
            }
        } else {
            for x in (k / 2 + 1..=n - k / 2).rev() {
                answer.push(x);
            }
        }

        answer
    }
}
