impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut ret = 1950;

        for year in 1950..=2050 {
            let population = logs
                .iter()
                .filter(|&log| (log[0]..log[1]).contains(&year))
                .count();

            if population > max {
                max = population;
                ret = year;
            }
        }

        ret
    }
}
