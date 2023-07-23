impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p = (1..=m).rev().collect::<Vec<i32>>();
        let mut ret = vec![0; queries.len()];

        for i in 0..queries.len() {
            let posi = p.iter().position(|&x| x == queries[i]).unwrap();
            let x = p.remove(posi);
            p.push(x);
            ret[i] = m - 1 - posi as i32;
        }

        ret
    }
}
