impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]];

        for num in nums {
            let mut temp = ret.clone();
            temp.iter_mut().for_each(|x| x.push(num));
            ret.append(&mut temp);
        }

        ret
    }
}
