# 2296. Design a Text Editor
Design a text editor with a cursor that can do the following:
* **Add** text to where the cursor is.
* **Delete** text from where the cursor is (simulating the backspace key).
* **Move** the cursor either left or right.

When deleting text, only characters to the left of the cursor will be deleted. The cursor will also remain within the actual text and cannot be moved beyond it. More formally, we have that `0 <= cursor.position <= currentText.length` always holds.

Implement the `TextEditor` class:
* `TextEditor()` Initializes the object with empty text.
* `void addText(string text)` Appends `text` to where the cursor is. The cursor ends to the right of `text`.
* `int deleteText(int k)` Deletes `k` characters to the left of the cursor. Returns the number of characters actually deleted.
* `string cursorLeft(int k)` Moves the cursor to the left `k` times. Returns the last `min(10, len)` characters to the left of the cursor, where `len` is the number of characters to the left of the cursor.
* `string cursorRight(int k)` Moves the cursor to the right `k` times. Returns the last `min(10, len)` characters to the left of the cursor, where `len` is the number of characters to the left of the cursor.

#### Example 1:
<pre>
<strong>Input:</strong>
["TextEditor", "addText", "deleteText", "addText", "cursorRight", "cursorLeft", "deleteText", "cursorLeft", "cursorRight"]
[[], ["leetcode"], [4], ["practice"], [3], [8], [10], [2], [6]]
<strong>Output:</strong>
[null, null, 4, null, "etpractice", "leet", 4, "", "practi"]
<strong>Explanation:</strong>
TextEditor textEditor = new TextEditor(); // The current text is "|". (The '|' character represents the cursor)
textEditor.addText("leetcode"); // The current text is "leetcode|".
textEditor.deleteText(4); // return 4
                          // The current text is "leet|".
                          // 4 characters were deleted.
textEditor.addText("practice"); // The current text is "leetpractice|".
textEditor.cursorRight(3); // return "etpractice"
                           // The current text is "leetpractice|".
                           // The cursor cannot be moved beyond the actual text and thus did not move.
                           // "etpractice" is the last 10 characters to the left of the cursor.
textEditor.cursorLeft(8); // return "leet"
                          // The current text is "leet|practice".
                          // "leet" is the last min(10, 4) = 4 characters to the left of the cursor.
textEditor.deleteText(10); // return 4
                           // The current text is "|practice".
                           // Only 4 characters were deleted.
textEditor.cursorLeft(2); // return ""
                          // The current text is "|practice".
                          // The cursor cannot be moved beyond the actual text and thus did not move.
                          // "" is the last min(10, 0) = 0 characters to the left of the cursor.
textEditor.cursorRight(6); // return "practi"
                           // The current text is "practi|ce".
                           // "practi" is the last min(10, 6) = 6 characters to the left of the cursor.
</pre>

#### Constraints:
* `1 <= text.length, k <= 40`
* `text` consists of lowercase English letters.
* At most <code>2 * 10<sup>4</sup></code> calls **in total** will be made to `addText`, `deleteText`, `cursorLeft` and `cursorRight`.

**Follow-up:** Could you find a solution with time complexity of `O(k)` per call?

## Solutions (Rust)

### 1. Solution
```Rust
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
```
