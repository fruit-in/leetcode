impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut occurs = [(false, false); 26];

        s.bytes().for_each(|c| match c {
            b'a'..=b'z' => occurs[(c - b'a') as usize].0 = true,
            _ => occurs[(c - b'A') as usize].1 = true,
        });

        match (0..26).rposition(|i| occurs[i] == (true, true)) {
            Some(i) => String::from_utf8(vec![i as u8 + b'A']).unwrap(),
            None => "".to_string(),
        }
    }
}
