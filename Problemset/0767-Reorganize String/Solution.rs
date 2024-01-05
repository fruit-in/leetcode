use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut count = [0; 26];
        let mut heap = BinaryHeap::new();
        let mut ret = vec![];

        for ch in s.bytes() {
            count[(ch - b'a') as usize] += 1;
        }
        for ch in b'a'..=b'z' {
            heap.push((count[(ch - b'a') as usize], ch));
        }

        for _ in 0..s.len() {
            let (count0, ch0) = heap.pop().unwrap();

            if *ret.last().unwrap_or(&0) != ch0 {
                ret.push(ch0);
                heap.push((count0 - 1, ch0));
            } else if heap.peek().unwrap().0 == 0 {
                return String::new();
            } else {
                let (count1, ch1) = heap.pop().unwrap();
                ret.push(ch1);
                heap.push((count1 - 1, ch1));
                heap.push((count0, ch0));
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
