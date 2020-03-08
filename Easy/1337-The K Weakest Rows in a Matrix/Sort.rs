impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mat = mat.iter().map(|r| r.iter().sum()).collect::<Vec<i32>>();
        let mut ret = (0..(mat.len() as i32)).collect::<Vec<i32>>();
        ret.sort_by_key(|&k| mat[k as usize]);

        ret[..(k as usize)].to_vec()
    }
}
