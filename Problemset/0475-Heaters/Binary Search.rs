impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut heaters = heaters;
        heaters.sort_unstable();

        let mut min_radius = 0;

        for house in houses {
            if let Err(i) = heaters.binary_search(&house) {
                min_radius = min_radius.max(
                    if i == 0 {
                        heaters[0] - house
                    } else if i == heaters.len() {
                        house - heaters[i - 1]
                    } else {
                        (heaters[i] - house).min(house - heaters[i - 1])
                    }
                );
            }
        }

        min_radius
    }
}
