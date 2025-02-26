# 2296. 设计一个文本编辑器
请你设计一个带光标的文本编辑器，它可以实现以下功能：
* **添加：**在光标所在处添加文本。
* **删除：**在光标所在处删除文本（模拟键盘的删除键）。
* **移动：**将光标往左或者往右移动。

当删除文本时，只有光标左边的字符会被删除。光标会留在文本内，也就是说任意时候 `0 <= cursor.position <= currentText.length` 都成立。

请你实现 `TextEditor` 类：
* `TextEditor()` 用空文本初始化对象。
* `void addText(string text)` 将 `text` 添加到光标所在位置。添加完后光标在 `text` 的右边。
* `int deleteText(int k)` 删除光标左边 `k` 个字符。返回实际删除的字符数目。
* `string cursorLeft(int k)` 将光标向左移动 `k` 次。返回移动后光标左边 `min(10, len)` 个字符，其中 `len` 是光标左边的字符数目。
* `string cursorRight(int k)` 将光标向右移动 `k` 次。返回移动后光标左边 `min(10, len)` 个字符，其中 `len` 是光标左边的字符数目。

#### 示例 1:
<pre>
<strong>输入:</strong>
["TextEditor", "addText", "deleteText", "addText", "cursorRight", "cursorLeft", "deleteText", "cursorLeft", "cursorRight"]
[[], ["leetcode"], [4], ["practice"], [3], [8], [10], [2], [6]]
<strong>输出:</strong>
[null, null, 4, null, "etpractice", "leet", 4, "", "practi"]
<strong>解释:</strong>
TextEditor textEditor = new TextEditor(); // 当前 text 为 "|" 。（'|' 字符表示光标）
textEditor.addText("leetcode"); // 当前文本为 "leetcode|" 。
textEditor.deleteText(4); // 返回 4
                          // 当前文本为 "leet|" 。
                          // 删除了 4 个字符。
textEditor.addText("practice"); // 当前文本为 "leetpractice|" 。
textEditor.cursorRight(3); // 返回 "etpractice"
                           // 当前文本为 "leetpractice|".
                           // 光标无法移动到文本以外，所以无法移动。
                           // "etpractice" 是光标左边的 10 个字符。
textEditor.cursorLeft(8); // 返回 "leet"
                          // 当前文本为 "leet|practice" 。
                          // "leet" 是光标左边的 min(10, 4) = 4 个字符。
textEditor.deleteText(10); // 返回 4
                           // 当前文本为 "|practice" 。
                           // 只有 4 个字符被删除了。
textEditor.cursorLeft(2); // 返回 ""
                          // 当前文本为 "|practice" 。
                          // 光标无法移动到文本以外，所以无法移动。
                          // "" 是光标左边的 min(10, 0) = 0 个字符。
textEditor.cursorRight(6); // 返回 "practi"
                           // 当前文本为 "practi|ce" 。
                           // "practi" 是光标左边的 min(10, 6) = 6 个字符。
</pre>

#### 提示:
* `1 <= text.length, k <= 40`
* `text` 只含有小写英文字母。
* 调用 `addText` ，`deleteText` ，`cursorLeft` 和 `cursorRight` 的 **总** 次数不超过 <code>2 * 10<sup>4</sup></code> 次。

**进阶：**你能设计并实现一个每次调用时间复杂度为 `O(k)` 的解决方案吗？

## 题解 (Rust)

### 1. 题解
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
