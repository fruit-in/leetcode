# 225. Implement Stack using Queues
Implement the following operations of a stack using queues.
* push(x) -- Push element x onto stack.
* pop() -- Removes the element on top of the stack.
* top() -- Get the top element.
* empty() -- Return whether the stack is empty.

#### Example:
<pre>
MyStack stack = new MyStack();

stack.push(1);
stack.push(2);
stack.top();   // returns 2
stack.pop();   // returns 2
stack.empty(); // returns false
</pre>

#### Note:
* You must use *only* standard operations of a queue -- which means only <code>push to back</code>, <code>peek/pop from front</code>, <code>size</code>, and <code>is empty</code> operations are valid.
* Depending on your language, queue may not be supported natively. You may simulate a queue by using a list or deque (double-ended queue), as long as you use only standard operations of a queue.
* You may assume that all operations are valid (for example, no pop or top operations will be called on an empty stack).

## Solutions (Rust)

### 1. One Queue
```Rust
struct MyStack {
    queue: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.queue.push(x);
        for _ in 1..self.queue.len() {
            let temp = self.queue.remove(0);
            self.queue.push(temp)
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.queue.remove(0)
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.queue.first().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
```
