impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        rec1[0].max(rec2[0]) < rec1[2].min(rec2[2]) && rec1[1].max(rec2[1]) < rec1[3].min(rec2[3])
    }
}
