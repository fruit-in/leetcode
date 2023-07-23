impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut abc = vec![a, b, c];
        let mut prev = 0;
        let mut v = vec![(a, 'a'), (b, 'b'), (c, 'c')];
        let mut ret = vec![];

        while let Some((mut x, ch)) = v.into_iter().filter(|t| t.0 > 0).max_by_key(|t| t.0) {
            ret.push(ch);
            x -= 1;
            if x >= prev && x > 0 {
                ret.push(ch);
                x -= 1;
            }

            abc[ch as usize - 97] = x;
            prev = x;
            v = match ch {
                'a' => vec![(abc[1], 'b'), (abc[2], 'c')],
                'b' => vec![(abc[0], 'a'), (abc[2], 'c')],
                _ => vec![(abc[0], 'a'), (abc[1], 'b')],
            };
        }

        ret.iter().collect()
    }
}
