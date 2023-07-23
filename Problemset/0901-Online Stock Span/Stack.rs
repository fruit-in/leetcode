struct StockSpanner {
    stack: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;

        while let Some((p, s)) = self.stack.pop() {
            if p <= price {
                span += s;
            } else {
                self.stack.push((p, s));
                break;
            }
        }

        self.stack.push((price, span));

        span
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
