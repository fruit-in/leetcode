impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        if let Some(ch) = chars.next() {
            if ch.is_ascii_uppercase() {
                if let Some(ch) = chars.next() {
                    if ch.is_ascii_uppercase() {
                        while let Some(ch) = chars.next() {
                            if ch.is_ascii_lowercase() {
                                return false;
                            }
                        }
                    } else {
                        while let Some(ch) = chars.next() {
                            if ch.is_ascii_uppercase() {
                                return false;
                            }
                        }
                    }
                }
            } else {
                while let Some(ch) = chars.next() {
                    if ch.is_ascii_uppercase() {
                        return false;
                    }
                }
            }
        }
        true
    }
}
