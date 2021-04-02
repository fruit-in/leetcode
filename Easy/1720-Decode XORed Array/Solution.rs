impl Solution {
    pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        let mut ret = vec![first];

        for n in encoded {
            first ^= n;
            ret.push(first);
        }

        ret
    }
}
