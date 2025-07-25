use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }

        let mut sum = target.iter().map(|&x| x as i64).sum::<i64>();
        let mut heap = target.iter().map(|&x| x as i64).collect::<BinaryHeap<_>>();

        while let Some(mut x) = heap.pop() {
            let y = *heap.peek().unwrap();
            let tmp = sum - x;
            x = y / tmp * tmp + x % tmp;
            if y > x {
                x += tmp;
            }
            sum = tmp + x;

            if x == 1 {
                return true;
            } else if 2 * x - sum < 1 {
                return false;
            }

            heap.push(2 * x - sum);
            sum = x;
        }

        unreachable!()
    }
}
