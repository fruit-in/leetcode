impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut xyz = vec![a, b, c];
        xyz.sort_unstable();
        let x = xyz[0];
        let y = xyz[1];
        let z = xyz[2];

        if z - x == 2 {
            vec![0, 0]
        } else if z - y > 2 && y - x > 2 {
            vec![2, z - x - 2]
        } else {
            vec![1, z - x - 2]
        }
    }
}
