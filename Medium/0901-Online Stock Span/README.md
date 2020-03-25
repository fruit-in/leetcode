# 901. Online Stock Span
Write a class ```StockSpanner``` which collects daily price quotes for some stock, and returns the *span* of that stock's price for the current day.

The span of the stock's price today is defined as the maximum number of consecutive days (starting from today and going backwards) for which the price of the stock was less than or equal to today's price.

For example, if the price of a stock over the next 7 days were ```[100, 80, 60, 70, 60, 75, 85]```, then the stock spans would be ```[1, 1, 1, 2, 1, 4, 6]```.

#### Example 1:
<pre>
<strong>Input:</strong>
["StockSpanner","next","next","next","next","next","next","next"], [[],[100],[80],[60],[70],[60],[75],[85]]
<strong>Output:</strong> [null,1,1,1,2,1,4,6]
<strong>Explanation:</strong>
First, S = StockSpanner() is initialized.  Then:
S.next(100) is called and returns 1,
S.next(80) is called and returns 1,
S.next(60) is called and returns 1,
S.next(70) is called and returns 2,
S.next(60) is called and returns 1,
S.next(75) is called and returns 4,
S.next(85) is called and returns 6.

Note that (for example) S.next(75) returned 4, because the last 4 prices
(including today's price of 75) were less than or equal to today's price.
</pre>

#### Note:
1. Calls to ```StockSpanner.next(int price)``` will have ```1 <= price <= 10^5```.
2. There will be at most ```10000``` calls to ```StockSpanner.next``` per test case.
3. There will be at most ```150000``` calls to ```StockSpanner.next``` across all test cases.
4. The total time limit for this problem has been reduced by 75% for C++, and 50% for all other languages.

## Solutions (Rust)

### 1. Stack
```Rust
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
```
