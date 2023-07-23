impl Solution {
    pub fn interpret(command: String) -> String {
        (command + " ")
            .as_bytes()
            .windows(2)
            .map(|w| match (w[0], w[1]) {
                (b'G', _) => "G",
                (b'(', b')') => "o",
                (b'(', b'a') => "al",
                _ => "",
            })
            .collect()
    }
}
