impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1{
            return n;
        }
        let mut pre1 = 1;
        let mut pre2 = 0;
        let mut fib_num = 1;
        for i in 2..=n {
            fib_num = pre1 + pre2;
            pre2 = pre1;
            pre1 = fib_num;
        }
        fib_num
    }
}
