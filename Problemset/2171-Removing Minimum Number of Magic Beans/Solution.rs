impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut lsum = 0;
        let mut rsum = beans.iter().sum::<i64>();
        let mut ret = i64::MAX;
        beans.sort_unstable();

        for i in 0..beans.len() {
            ret = ret.min(lsum + rsum - (beans.len() - i) as i64 * beans[i]);
            lsum += beans[i];
            rsum -= beans[i];
        }

        ret
    }
}
