impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parent = (0..26).collect::<Vec<usize>>();

        for i in 0..equations.len() {
            let equation = equations[i].as_bytes();
            let mut x = (equation[0] - b'a') as usize;
            let mut y = (equation[3] - b'a') as usize;

            if equation[1] == b'=' {
                while parent[x] != x {
                    x = parent[x];
                }
                while parent[y] != y {
                    y = parent[y];
                }
                if x > y {
                    parent[x] = y;
                } else {
                    parent[y] = x;
                }
            }
        }

        for i in 0..26 {
            while parent[i] != parent[parent[i]] {
                parent[i] = parent[parent[i]];
            }
        }

        for i in 0..equations.len() {
            let equation = equations[i].as_bytes();
            let x = (equation[0] - b'a') as usize;
            let y = (equation[3] - b'a') as usize;

            if equation[1] == b'!' && parent[x] == parent[y] {
                return false;
            }
        }

        true
    }
}
