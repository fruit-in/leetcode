impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut i = 1;
        let mut ret = vec![];

        for num in target {
            for _ in 0..(num - i) {
                ret.push("Push".to_string());
                ret.push("Pop".to_string());
            }
            ret.push("Push".to_string());
            i = num + 1;
        }

        ret
    }
}
