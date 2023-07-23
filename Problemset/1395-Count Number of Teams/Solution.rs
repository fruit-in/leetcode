impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut ret = 0;

        for j in 1..(rating.len() - 1) {
            let ltl = rating[..j].iter().filter(|&&x| x < rating[j]).count();
            let gtl = j - ltl;
            let ltr = rating[j..].iter().filter(|&&x| x < rating[j]).count();
            let gtr = rating.len() - 1 - j - ltr;

            ret += ltl * gtr + ltr * gtl;
        }

        ret as i32
    }
}
