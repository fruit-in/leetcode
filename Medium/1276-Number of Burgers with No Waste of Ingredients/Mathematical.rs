impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 == 0
            && tomato_slices >= 2 * cheese_slices
            && tomato_slices <= 4 * cheese_slices
        {
            vec![
                tomato_slices / 2 - cheese_slices,
                2 * cheese_slices - tomato_slices / 2,
            ]
        } else {
            vec![]
        }
    }
}
