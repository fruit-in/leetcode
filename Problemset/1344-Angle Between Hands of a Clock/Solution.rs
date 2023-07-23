impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let m_angle = minutes as f64 * 6.;
        let h_angle = (hour % 12 * 60 + minutes) as f64 * 0.5;
        let min = m_angle.min(h_angle);
        let max = m_angle.max(h_angle);

        (max - min).min(360. - max + min)
    }
}
