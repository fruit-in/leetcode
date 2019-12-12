# 703. Kth Largest Element in a Stream
Design a class to find the **k**th largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.

Your ```KthLargest``` class will have a constructor which accepts an integer ```k``` and an integer array ```nums```, which contains initial elements from the stream. For each call to the method ```KthLargest.add```, return the element representing the kth largest element in the stream.

#### Example:
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

#### Note:
You may assume that ```nums```' length ≥ ```k-1``` and ```k``` ≥ 1.

## Solutions (Rust)

### 1. Sort
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

### 2. Min Heap
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
