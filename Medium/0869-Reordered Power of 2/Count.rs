impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut power_of2 = Vec::new();

        for i in 0..30 {
            let mut arr = vec![0; 10];
            let mut x = 2_usize.pow(i);
            while x > 0 {
                arr[x % 10] += 1;
                x /= 10;
            }
            power_of2.push(arr);
        }

        let mut arr = vec![0; 10];
        let mut n = n as usize;
        while n > 0 {
            arr[n % 10] += 1;
            n /= 10;
        }

        power_of2.contains(&arr)
    }
}
