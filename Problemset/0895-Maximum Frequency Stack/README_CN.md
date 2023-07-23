# 895. 最大频率栈
实现 ```FreqStack```，模拟类似栈的数据结构的操作的一个类。

```FreqStack``` 有两个函数：
* ```push(int x)```，将整数 ```x``` 推入栈中。
* ```pop()```，它**移除**并返回栈中出现最频繁的元素。
    * 如果最频繁的元素不只一个，则移除并返回最接近栈顶的元素。

#### 示例:
<pre>
<strong>输入:</strong>
["FreqStack","push","push","push","push","push","push","pop","pop","pop","pop"],
[[],[5],[7],[5],[7],[4],[5],[],[],[],[]]
<strong>输出:</strong> [null,null,null,null,null,null,null,5,7,5,4]
<strong>解释:</strong>
执行六次 .push 操作后，栈自底向上为 [5,7,5,7,4,5]。然后：

pop() -> 返回 5，因为 5 是出现频率最高的。
栈变成 [5,7,5,7,4]。

pop() -> 返回 7，因为 5 和 7 都是频率最高的，但 7 最接近栈顶。
栈变成 [5,7,5,4]。

pop() -> 返回 5 。
栈变成 [5,7,4]。

pop() -> 返回 4 。
栈变成 [5,7]。
</pre>

#### 提示:
* 对 ```FreqStack.push(int x)``` 的调用中 ```0 <= x <= 10^9```。
* 如果栈的元素数目为零，则保证不会调用  ```FreqStack.pop()```。
* 单个测试样例中，对 ```FreqStack.push``` 的总调用次数不会超过 ```10000```。
* 单个测试样例中，对 ```FreqStack.pop``` 的总调用次数不会超过 ```10000```。
* 所有测试样例中，对 ```FreqStack.push``` 和 ```FreqStack.pop``` 的总调用次数不会超过 ```150000```。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

struct FreqStack {
    num_freq: HashMap<i32, i32>,
    freq_stack: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self {
            num_freq: HashMap::new(),
            freq_stack: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let freq = self.num_freq.entry(x).or_default();
        *freq += 1;
        self.freq_stack.entry(*freq).or_default().push(x);
        self.max_freq = self.max_freq.max(*freq);
    }

    fn pop(&mut self) -> i32 {
        let stack = self.freq_stack.get_mut(&self.max_freq).unwrap();
        let x = stack.pop().unwrap();
        *self.num_freq.get_mut(&x).unwrap() -= 1;
        if stack.is_empty() {
            self.max_freq -= 1;
        }
        x
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */
```
