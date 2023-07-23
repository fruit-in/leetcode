impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut min_radius = 0;

        for house in &houses {
            let mut min_distance = std::i32::MAX;

            for heater in &heaters {
                min_distance = min_distance.min((heater - house).abs());
            }

            min_radius = min_radius.max(min_distance);
        }

        min_radius
    }
}
