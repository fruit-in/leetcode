impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut indices = vec![];

        for i in (0..nums.len()).rev() {
            match indices.binary_search_by_key(&nums[i], |&j| nums[j]) {
                Ok(k) if k == indices.len() - 1 => indices[k] = i,
                Ok(k) => {
                    nums.swap(i, indices[k + 1]);
                    let mut tmp = nums.split_off(i + 1);
                    tmp.sort_unstable();
                    nums.append(&mut tmp);
                    return;
                }
                Err(k) if k == indices.len() => indices.push(i),
                Err(k) => {
                    nums.swap(i, indices[k]);
                    let mut tmp = nums.split_off(i + 1);
                    tmp.sort_unstable();
                    nums.append(&mut tmp);
                    return;
                }
            }
        }

        nums.reverse();
    }
}
