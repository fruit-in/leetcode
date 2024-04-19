use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        let mut free = (-(n - 1)..=0).collect::<BinaryHeap<_>>();
        let mut used = BinaryHeap::<(i64, i32)>::new();
        let mut time = 0_i64;
        let mut count = vec![0; n as usize];

        meetings.sort_unstable();

        for meeting in &meetings {
            let (start, end) = (meeting[0] as i64, meeting[1] as i64);

            if free.is_empty() {
                time = time.max(-used.peek().unwrap().0);
            }
            time = time.max(start);

            while let Some(&(t, r)) = used.peek() {
                if -t <= time {
                    free.push(r);
                    used.pop();
                } else {
                    break;
                }
            }

            used.push((start - end - time, *free.peek().unwrap()));
            count[(-free.pop().unwrap()) as usize] += 1;
        }

        (0..n).max_by_key(|&r| (count[r as usize], -r)).unwrap()
    }
}
