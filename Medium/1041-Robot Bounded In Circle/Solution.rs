impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut position = (0, 0);
        let mut direction = 0;

        for ins in instructions.chars() {
            match (ins, direction % 4) {
                ('G', 0) => position.1 += 1,
                ('G', 1) => position.0 += 1,
                ('G', 2) => position.1 -= 1,
                ('G', 3) => position.0 -= 1,
                ('L', _) => direction += 3,
                _ => direction += 1,
            }
        }

        position == (0, 0) || direction % 4 != 0
    }
}
