# 1472. Design Browser History
You have a **browser** of one tab where you start on the `homepage` and you can visit another `url`, get back in the history number of `steps` or move forward in the history number of `steps`.

Implement the `BrowserHistory` class:
* `BrowserHistory(string homepage)` Initializes the object with the `homepage` of the browser.
* `void visit(string url)` Visits `url` from the current page. It clears up all the forward history.
* `string back(int steps)` Move `steps` back in history. If you can only return `x` steps in the history and `steps > x`, you will return only `x` steps. Return the current `url` after moving back in history **at most** `steps`.
* `string forward(int steps)` Move `steps` forward in history. If you can only forward `x` steps in the history and `steps > x`, you will forward only `x` steps. Return the current `url` after forwarding in history **at most** `steps`.

#### Example:
<pre>
<strong>Input:</strong>
["BrowserHistory","visit","visit","visit","back","back","forward","visit","forward","back","back"]
[["leetcode.com"],["google.com"],["facebook.com"],["youtube.com"],[1],[1],[1],["linkedin.com"],[2],[2],[7]]
<strong>Output:</strong>
[null,null,null,null,"facebook.com","google.com","facebook.com",null,"linkedin.com","google.com","leetcode.com"]
<strong>Explanation:</strong>
BrowserHistory browserHistory = new BrowserHistory("leetcode.com");
browserHistory.visit("google.com");       // You are in "leetcode.com". Visit "google.com"
browserHistory.visit("facebook.com");     // You are in "google.com". Visit "facebook.com"
browserHistory.visit("youtube.com");      // You are in "facebook.com". Visit "youtube.com"
browserHistory.back(1);                   // You are in "youtube.com", move back to "facebook.com" return "facebook.com"
browserHistory.back(1);                   // You are in "facebook.com", move back to "google.com" return "google.com"
browserHistory.forward(1);                // You are in "google.com", move forward to "facebook.com" return "facebook.com"
browserHistory.visit("linkedin.com");     // You are in "facebook.com". Visit "linkedin.com"
browserHistory.forward(2);                // You are in "linkedin.com", you cannot move forward any steps.
browserHistory.back(2);                   // You are in "linkedin.com", move back two steps to "facebook.com" then to "google.com". return "google.com"
browserHistory.back(7);                   // You are in "google.com", you can move back only one step to "leetcode.com". return "leetcode.com"
</pre>

#### Constraints:
* `1 <= homepage.length <= 20`
* `1 <= url.length <= 20`
* `1 <= steps <= 100`
* `homepage` and `url` consist of  '.' or lower case English letters.
* At most `5000` calls will be made to `visit`, `back`, and `forward`.

## Solutions (Rust)

### 1. Solution
```Rust
struct BrowserHistory {
    history: Vec<String>,
    current: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            current: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.history.truncate(self.current + 1);
        self.history.push(url);
        self.current += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.current = self.current.saturating_sub(steps as usize);

        self.history[self.current].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.current = (self.current + steps as usize).min(self.history.len() - 1);

        self.history[self.current].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
```
