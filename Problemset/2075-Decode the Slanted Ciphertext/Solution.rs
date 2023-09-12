impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let cols = encoded_text.len() / rows;
        let encoded_text = encoded_text.as_bytes();
        let mut original_text = vec![];

        for col in 0..cols {
            for row in 0..rows {
                if col + row >= cols {
                    break;
                }

                original_text.push(encoded_text[row * cols + col + row]);
            }
        }

        String::from_utf8(original_text)
            .unwrap()
            .trim_end()
            .to_string()
    }
}
