impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let count0 = binary.bytes().filter(|&b| b == b'0').count();
        let count1 = binary.bytes().take_while(|&b| b == b'1').count();
        let mut binary = binary.into_bytes();

        if count0 > 1 {
            binary = vec![b'1'; binary.len()];
            binary[count0 + count1 - 1] = b'0';
        }

        String::from_utf8(binary).unwrap()
    }
}
