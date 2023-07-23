impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut magical = vec![];
        let mut elem = true;
        let mut ret = 0;

        for i in 0..(n as usize) {
            magical.push(elem);
            match magical[i] {
                true => ret += 1,
                false => magical.push(elem),
            }
            elem = !elem;
        }

        ret
    }
}
