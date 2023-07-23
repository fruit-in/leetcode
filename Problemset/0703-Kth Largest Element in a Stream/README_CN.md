# 703. 数据流中的第K大元素
设计一个找到数据流中第K大元素的类（class）。注意是排序后的第K大元素，不是第K个不同的元素。

你的 ```KthLargest``` 类需要一个同时接收整数 ```k``` 和整数数组```nums``` 的构造器，它包含数据流中的初始元素。每次调用 ```KthLargest.add```，返回当前数据流中第K大的元素。

#### 示例:
```
int k = 3;
int[] arr = [4,5,8,2];
KthLargest kthLargest = new KthLargest(3, arr);
kthLargest.add(3);   // returns 4
kthLargest.add(5);   // returns 5
kthLargest.add(10);  // returns 5
kthLargest.add(9);   // returns 8
kthLargest.add(4);   // returns 8
```

#### 说明:
你可以假设 ```nums``` 的长度≥ ```k-1``` 且```k``` ≥ 1。

## 题解 (Rust)

### 1. 排序
```Rust
struct KthLargest {
    k: usize,
    elements: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k: k as usize,
            elements: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.elements.push(val);
        self.elements.sort_unstable_by(|a, b| b.cmp(a));
        self.elements[self.k - 1]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
```

### 2. 最小堆
```Rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    elements: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = Self {
            k: k as usize,
            elements: BinaryHeap::with_capacity(k as usize),
        };

        for num in nums {
            kth_largest.add(num);
        }

        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.elements.len() < self.k {
            self.elements.push(Reverse(val));
        } else if self.elements.peek() > Some(&Reverse(val)) {
            self.elements.pop();
            self.elements.push(Reverse(val));
        }

        match self.elements.peek() {
            Some(&Reverse(x)) => x,
            None => 0,
        }
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
```
