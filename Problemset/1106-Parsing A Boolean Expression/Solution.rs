impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut ops = vec![];
        let mut bools = vec![];

        for ch in expression.chars() {
            match ch {
                '!' | '&' | '|' => ops.push(ch),
                't' | 'f' => bools.push(Some(ch == 't')),
                ')' => match ops.pop() {
                    Some('!') => {
                        let tmp = !bools.pop().unwrap().unwrap();
                        bools.pop();
                        bools.push(Some(tmp));
                    }
                    Some('&') => {
                        let mut tmp = true;
                        while let Some(Some(b)) = bools.pop() {
                            tmp &= b;
                        }
                        bools.push(Some(tmp));
                    }
                    Some('|') => {
                        let mut tmp = false;
                        while let Some(Some(b)) = bools.pop() {
                            tmp |= b;
                        }
                        bools.push(Some(tmp));
                    }
                    _ => {
                        let tmp = bools.pop().unwrap();
                        bools.pop();
                        bools.push(tmp);
                    }
                },
                '(' => bools.push(None),
                _ => (),
            }
        }

        bools[0].unwrap()
    }
}
