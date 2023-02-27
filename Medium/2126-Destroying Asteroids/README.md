# 2126. Destroying Asteroids
You are given an integer `mass`, which represents the original mass of a planet. You are further given an integer array `asteroids`, where `asteroids[i]` is the mass of the <code>i<sup>th</sup></code> asteroid.

You can arrange for the planet to collide with the asteroids in **any arbitrary order**. If the mass of the planet is **greater than or equal to** the mass of the asteroid, the asteroid is **destroyed** and the planet **gains** the mass of the asteroid. Otherwise, the planet is destroyed.

Return `true` *if **all** asteroids can be destroyed. Otherwise, return* `false`.

#### Example 1:
<pre>
<strong>Input:</strong> mass = 10, asteroids = [3,9,19,5,21]
<strong>Output:</strong> true
<strong>Explanation:</strong> One way to order the asteroids is [9,19,5,3,21]:
- The planet collides with the asteroid with a mass of 9. New planet mass: 10 + 9 = 19
- The planet collides with the asteroid with a mass of 19. New planet mass: 19 + 19 = 38
- The planet collides with the asteroid with a mass of 5. New planet mass: 38 + 5 = 43
- The planet collides with the asteroid with a mass of 3. New planet mass: 43 + 3 = 46
- The planet collides with the asteroid with a mass of 21. New planet mass: 46 + 21 = 67
All asteroids are destroyed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> mass = 5, asteroids = [4,9,23,4]
<strong>Output:</strong> false
<strong>Explanation:</strong>
The planet cannot ever gain enough mass to destroy the asteroid with a mass of 23.
After the planet destroys the other asteroids, it will have a mass of 5 + 4 + 9 + 4 = 22.
This is less than 23, so a collision would not destroy the last asteroid.
</pre>

#### Constraints:
* <code>1 <= mass <= 10<sup>5</sup></code>
* <code>1 <= asteroids.length <= 10<sup>5</sup></code>
* <code>1 <= asteroids[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        let mut asteroids = asteroids;
        asteroids.sort_unstable();

        for asteroid in asteroids {
            if mass >= asteroid as i64 {
                mass += asteroid as i64;
            } else {
                return false;
            }
        }

        true
    }
}
```
