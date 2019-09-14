impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let mut counterclockwise = 0;
        curr = destination;
        while curr != start {
            counterclockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }
        clockwise.min(counterclockwise)
    }
}
