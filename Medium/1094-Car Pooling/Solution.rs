impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pick = [0; 1001];

        for trip in trips {
            pick[trip[2] as usize] -= trip[0];
            pick[trip[1] as usize] += trip[0];
        }

        for i in 1..1001 {
            pick[i] += pick[i - 1];
            if pick[i] > capacity {
                return false;
            }
        }

        pick[0] <= capacity
    }
}
