impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut rotate_top = [0, 1];
        let mut rotate_bottom = [0, 1];

        for i in 1..tops.len() {
            if rotate_top[0] >= 0 && tops[i] != tops[0] && bottoms[i] == tops[0] {
                rotate_top[0] += 1;
            } else if tops[i] != tops[0] {
                rotate_top[0] = -1;
            }
            if rotate_top[1] >= 0 && bottoms[i] != tops[0] && tops[i] == tops[0] {
                rotate_top[1] += 1;
            } else if bottoms[i] != tops[0] {
                rotate_top[1] = -1;
            }
            if rotate_bottom[0] >= 0 && bottoms[i] != bottoms[0] && tops[i] == bottoms[0] {
                rotate_bottom[0] += 1;
            } else if bottoms[i] != bottoms[0] {
                rotate_bottom[0] = -1;
            }
            if rotate_bottom[1] >= 0 && tops[i] != bottoms[0] && bottoms[i] == bottoms[0] {
                rotate_bottom[1] += 1;
            } else if tops[i] != bottoms[0] {
                rotate_bottom[1] = -1;
            }
        }

        *rotate_top
            .iter()
            .chain(rotate_bottom.iter())
            .filter(|&&x| x >= 0)
            .min()
            .unwrap_or(&-1)
    }
}
