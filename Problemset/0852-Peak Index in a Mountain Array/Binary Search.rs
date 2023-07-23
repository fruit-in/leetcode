impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut head: usize = 0;
        let mut tail = a.len() - 1;
        let mut mid: usize;
        loop {
            mid = (head + tail) / 2;
            if a[mid - 1] < a[mid] && a[mid] > a[mid + 1] {
                return mid as i32;
            } else if a[mid] < a[mid + 1] {
                head = mid;
            } else if a[mid] > a[mid + 1] {
                tail = mid;
            }
        }
    }
}
