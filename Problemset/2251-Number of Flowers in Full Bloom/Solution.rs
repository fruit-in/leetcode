use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut flowers = flowers;
        let mut people = people
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect::<Vec<_>>();
        let mut flowers_heap = BinaryHeap::new();
        let mut i = 0;
        let mut answer = vec![0; people.len()];

        flowers.sort_unstable();
        people.sort_unstable();

        for &(p, j) in &people {
            while let Some(&Reverse(end)) = flowers_heap.peek() {
                if end >= p {
                    break;
                }

                flowers_heap.pop();
            }

            while i < flowers.len() && flowers[i][0] <= p {
                if flowers[i][1] >= p {
                    flowers_heap.push(Reverse(flowers[i][1]));
                }

                i += 1;
            }

            answer[j] = flowers_heap.len() as i32;
        }

        answer
    }
}
