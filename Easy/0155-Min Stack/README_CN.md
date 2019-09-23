# 155. 最小栈
设计一个支持 push，pop，top 操作，并能在常数时间内检索到最小元素的栈。
* push(x) -- 将元素 x 推入栈中。
* pop() -- 删除栈顶的元素。
* top() -- 获取栈顶元素。
* getMin() -- 检索栈中的最小元素。

#### 示例:
<pre>
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.
</pre>

## 题解 (Rust)

### 1. 两个栈
```Rust
struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.min.len() == 0 || x <= self.get_min() {
            self.min.push(x);
        }
        self.data.push(x);
    }

    fn pop(&mut self) {
        if self.top() == self.get_min() {
            self.min.pop();
        }
        self.data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
```

### 2. 一个栈
```Rust
struct MinStack {
    data: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.data.len() == 0 || x <= self.get_min() {
            self.data.push((x, x));
        } else {
            self.data.push((x, self.get_min()));
        }
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
```
