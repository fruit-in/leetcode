use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let mut ladders = ladders;
        let mut heap = BinaryHeap::new();

        for i in 0..heights.len() - 1 {
            let diff = heights[i + 1] - heights[i];

            if diff > 0 {
                heap.push(diff);

                if bricks < diff {
                    if ladders > 0 {
                        bricks += heap.pop().unwrap();
                        ladders -= 1;
                    } else {
                        return i as i32;
                    }
                }

                bricks -= diff;
            }
        }

        heights.len() as i32 - 1
    }
}
