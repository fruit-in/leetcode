struct TextEditor {
    strl: String,
    strr: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    fn new() -> Self {
        Self {
            strl: String::new(),
            strr: String::new(),
        }
    }

    fn add_text(&mut self, text: String) {
        self.strl.push_str(&text);
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let ret = k.min(self.strl.len() as i32);

        self.strl
            .truncate(self.strl.len() - self.strl.len().min(k as usize));

        ret
    }

    fn cursor_left(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.strl.pop() {
                self.strr.push(c);
            }
        }

        self.strl
            .get(self.strl.len() - self.strl.len().min(10)..)
            .unwrap()
            .to_string()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.strr.pop() {
                self.strl.push(c);
            }
        }

        self.strl
            .get(self.strl.len() - self.strl.len().min(10)..)
            .unwrap()
            .to_string()
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */
