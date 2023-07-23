impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut triplet = vec![0; 3];

        for tri in &triplets {
            if tri[0] <= target[0] && tri[1] <= target[1] && tri[2] <= target[2] {
                triplet[0] = triplet[0].max(tri[0]);
                triplet[1] = triplet[1].max(tri[1]);
                triplet[2] = triplet[2].max(tri[2]);
            }
        }

        triplet == target
    }
}
