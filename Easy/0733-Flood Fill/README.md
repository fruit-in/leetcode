# 733. Flood Fill
An ```image``` is represented by a 2-D array of integers, each integer representing the pixel value of the image (from 0 to 65535).

Given a coordinate ```(sr, sc)``` representing the starting pixel (row and column) of the flood fill, and a pixel value ```newColor```, "flood fill" the image.

To perform a "flood fill", consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color as the starting pixel), and so on. Replace the color of all of the aforementioned pixels with the newColor.

At the end, return the modified image.

#### Example 1:
<pre>
<strong>Input:</strong>
image = [[1,1,1],[1,1,0],[1,0,1]]
sr = 1, sc = 1, newColor = 2
<strong>Output:</strong> [[2,2,2],[2,2,0],[2,0,1]]
<strong>Explanation:</strong>
From the center of the image (with position (sr, sc) = (1, 1)), all pixels connected
by a path of the same color as the starting pixel are colored with the new color.
Note the bottom corner is not colored 2, because it is not 4-directionally connected
to the starting pixel.
</pre>

#### Note:
* The length of ```image``` and ```image[0]``` will be in the range ```[1, 50]```.
* The given starting pixel will satisfy ```0 <= sr < image.length``` and ```0 <= sc < image[0].length```.
* The value of each color in ```image[i][j]``` and ```newColor``` will be an integer in ```[0, 65535]```.

## Solutions (Rust)

### 1. DFS
```Rust
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
```
