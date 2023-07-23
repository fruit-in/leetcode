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
