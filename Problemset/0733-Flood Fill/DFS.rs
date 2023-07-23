impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let usr = sr as usize;
        let usc = sc as usize;
        let mut old_color = image[usr][usc];
        image[usr][usc] = new_color;

        if old_color != new_color {
            if sr > 0 && image[usr - 1][usc] == old_color {
                image = Self::flood_fill(image, sr - 1, sc, new_color);
            }
            if sc > 0 && image[usr][usc - 1] == old_color {
                image = Self::flood_fill(image, sr, sc - 1, new_color);
            }
            if usr < image.len() - 1 && image[usr + 1][usc] == old_color {
                image = Self::flood_fill(image, sr + 1, sc, new_color);
            }
            if usc < image[0].len() - 1 && image[usr][usc + 1] == old_color {
                image = Self::flood_fill(image, sr, sc + 1, new_color);
            }
        }

        image
    }
}
