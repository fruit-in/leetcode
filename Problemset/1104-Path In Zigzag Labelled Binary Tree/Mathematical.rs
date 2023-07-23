impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut x = 2_i32.pow((label as f64).log2() as u32);
        let mut label = label;
        let mut ret = vec![label];

        while label > 1 {
            label = x - 1 - ((label / 2) % (x / 2));
            x /= 2;
            ret.push(label);
        }

        ret.reverse();
        ret
    }
}
