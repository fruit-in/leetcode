use std::collections::VecDeque;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut distances1 = vec![-1; n];
        let mut distances2 = vec![-1; n];
        let mut i = node1 as usize;
        let mut d = 0;
        let mut min_d = i32::MAX;
        let mut ret = -1;

        while distances1[i] == -1 {
            distances1[i] = d;
            if edges[i] == -1 {
                break;
            }
            i = edges[i] as usize;
            d += 1;
        }

        i = node2 as usize;
        d = 0;

        while distances2[i] == -1 {
            distances2[i] = d;
            if edges[i] == -1 {
                break;
            }
            i = edges[i] as usize;
            d += 1;
        }

        for i in 0..n {
            if distances1[i] != -1 && distances2[i] != -1 {
                let d = distances1[i].max(distances2[i]);

                if ret == -1 || d < min_d {
                    min_d = d;
                    ret = i as i32;
                }
            }
        }

        ret
    }
}
