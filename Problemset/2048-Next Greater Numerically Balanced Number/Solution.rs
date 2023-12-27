impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut x = n + 1;

        loop {
            let mut count = [0; 10];
            let mut y = x as usize;

            while y > 0 {
                count[y % 10] += 1;
                y /= 10;
            }

            if (0..10).all(|i| count[i] == i || count[i] == 0) {
                return x;
            }

            x += 1;
        }

        unreachable!()
    }
}
