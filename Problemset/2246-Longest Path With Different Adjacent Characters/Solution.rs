impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count = vec![0; n];
        let mut nodes = vec![];
        let mut max2 = vec![(0, 0); n];
        let mut ret = 1;

        for i in 1..n {
            count[parent[i] as usize] += 1;
        }
        for i in 0..n {
            if count[i] == 0 {
                nodes.push(i);
            }
        }

        while let Some(i) = nodes.pop() {
            ret = ret.max(max2[i].0 + max2[i].1 + 1);

            if i > 0 {
                if s[i] != s[parent[i] as usize] {
                    if max2[i].0 + 1 >= max2[parent[i] as usize].0 {
                        max2[parent[i] as usize] = (max2[i].0 + 1, max2[parent[i] as usize].0);
                    } else if max2[i].0 + 1 > max2[parent[i] as usize].1 {
                        max2[parent[i] as usize].1 = max2[i].0 + 1;
                    }
                }

                count[parent[i] as usize] -= 1;
                if count[parent[i] as usize] == 0 {
                    nodes.push(parent[i] as usize);
                }
            }
        }

        ret
    }
}
