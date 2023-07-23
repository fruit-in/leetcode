impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();
        let mut balls = 0;
        let mut prev = 0;
        let mut ret = vec![0; boxes.len()];

        for i in 0..boxes.len() {
            ret[i] += prev + balls;
            prev += balls;
            if boxes[i] == b'1' {
                balls += 1;
            }
        }

        balls = 0;
        prev = 0;

        for i in (0..boxes.len()).rev() {
            ret[i] += prev + balls;
            prev += balls;
            if boxes[i] == b'1' {
                balls += 1;
            }
        }

        ret
    }
}
