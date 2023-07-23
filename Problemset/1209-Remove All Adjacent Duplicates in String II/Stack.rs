impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(u8, i32)> = vec![];

        for ch in s.bytes() {
            match stack.last_mut() {
                Some(last) if ch == last.0 => {
                    last.1 += 1;
                    if last.1 == k {
                        stack.pop();
                    }
                }
                _ => stack.push((ch, 1)),
            }
        }

        String::from_utf8(
            stack
                .into_iter()
                .map(|(ch, cnt)| vec![ch; cnt as usize])
                .flatten()
                .collect(),
        )
        .unwrap()
    }
}
