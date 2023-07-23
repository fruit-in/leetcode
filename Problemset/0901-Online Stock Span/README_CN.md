# 901. 股票价格跨度
编写一个 ```StockSpanner``` 类，它收集某些股票的每日报价，并返回该股票当日价格的跨度。

今天股票价格的跨度被定义为股票价格小于或等于今天价格的最大连续日数（从今天开始往回数，包括今天）。

例如，如果未来7天股票的价格是 ```[100, 80, 60, 70, 60, 75, 85]```，那么股票跨度将是 ```[1, 1, 1, 2, 1, 4, 6]```。

#### 示例:
<pre>
<strong>输入:</strong>
["StockSpanner","next","next","next","next","next","next","next"], [[],[100],[80],[60],[70],[60],[75],[85]]
<strong>输出:</strong> [null,1,1,1,2,1,4,6]
<strong>解释:</strong>
首先，初始化 S = StockSpanner()，然后：
S.next(100) 被调用并返回 1，
S.next(80) 被调用并返回 1，
S.next(60) 被调用并返回 1，
S.next(70) 被调用并返回 2，
S.next(60) 被调用并返回 1，
S.next(75) 被调用并返回 4，
S.next(85) 被调用并返回 6。

注意 (例如) S.next(75) 返回 4，因为截至今天的最后 4 个价格
(包括今天的价格 75) 小于或等于今天的价格。
</pre>

#### 提示:
1. 调用 ```StockSpanner.next(int price)``` 时，将有 ```1 <= price <= 10^5```。
2. 每个测试用例最多可以调用 ```10000``` 次 ```StockSpanner.next```。
3. 在所有测试用例中，最多调用 ```150000``` 次 ```StockSpanner.next```。
4. 此问题的总时间限制减少了 50%。

## 题解 (Rust)

### 1. 栈
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
