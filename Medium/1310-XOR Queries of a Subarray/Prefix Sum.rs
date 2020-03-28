impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = arr;
        let mut ret = Vec::new();

        for i in 1..arr.len() {
            arr[i] ^= arr[i - 1];
        }

        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize;
            ret.push(arr.get(l - 1).unwrap_or(&0) ^ arr[r]);
        }

        ret
    }
}
