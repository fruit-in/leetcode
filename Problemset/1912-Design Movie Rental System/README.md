# 1912. Design Movie Rental System
You have a movie renting company consisting of `n` shops. You want to implement a renting system that supports searching for, booking, and returning movies. The system should also support generating a report of the currently rented movies.

Each movie is given as a 2D integer array `entries` where <code>entries[i] = [shop<sub>i</sub>, movie<sub>i</sub>, price<sub>i</sub>]</code> indicates that there is a copy of movie <code>movie<sub>i</sub></code> at shop <code>shop<sub>i</sub></code> with a rental price of <code>price<sub>i</sub></code>. Each shop carries **at most one** copy of a movie <code>movie<sub>i</sub></code>.

The system should support the following functions:

* **Search:** Finds the **cheapest 5 shops** that have an **unrented copy** of a given movie. The shops should be sorted by **price** in ascending order, and in case of a tie, the one with the **smaller** <code>shop<sub>i</sub></code> should appear first. If there are less than 5 matching shops, then all of them should be returned. If no shop has an unrented copy, then an empty list should be returned.
* **Rent:** Rents an **unrented copy** of a given movie from a given shop.
* **Drop:** Drops off a **previously rented copy** of a given movie at a given shop.
* **Report:** Returns the **cheapest 5 rented movies** (possibly of the same movie ID) as a 2D list `res` where <code>res[j] = [shop<sub>j</sub>, movie<sub>j</sub>]</code> describes that the <code>j<sup>th</sup></code> cheapest rented movie <code>movie<sub>j</sub></code> was rented from the shop <code>shop<sub>j</sub></code>. The movies in `res` should be sorted by **price** in ascending order, and in case of a tie, the one with the **smaller** <code>shop<sub>j</sub></code> should appear first, and if there is still tie, the one with the **smaller** <code>movie<sub>j</sub></code> should appear first. If there are fewer than 5 rented movies, then all of them should be returned. If no movies are currently being rented, then an empty list should be returned.

Implement the `MovieRentingSystem` class:

* `MovieRentingSystem(int n, int[][] entries)` Initializes the `MovieRentingSystem` object with `n` shops and the movies in `entries`.
* `List<Integer> search(int movie)` Returns a list of shops that have an **unrented copy** of the given `movie` as described above.
* `void rent(int shop, int movie)` Rents the given `movie` from the given `shop`.
* `void drop(int shop, int movie)` Drops off a previously rented `movie` at the given `shop`.
* `List<List<Integer>> report()` Returns a list of cheapest **rented** movies as described above.

**Note:** The test cases will be generated such that `rent` will only be called if the shop has an **unrented** copy of the movie, and `drop` will only be called if the shop had **previously rented** out the movie.

#### Example 1:
<pre>
<strong>Input:</strong>
["MovieRentingSystem", "search", "rent", "rent", "report", "drop", "search"]
[[3, [[0, 1, 5], [0, 2, 6], [0, 3, 7], [1, 1, 4], [1, 2, 7], [2, 1, 5]]], [1], [0, 1], [1, 2], [], [1, 2], [2]]
<strong>Output:</strong>
[null, [1, 0, 2], null, null, [[0, 1], [1, 2]], null, [0, 1]]
<strong>Explanation:</strong>
MovieRentingSystem movieRentingSystem = new MovieRentingSystem(3, [[0, 1, 5], [0, 2, 6], [0, 3, 7], [1, 1, 4], [1, 2, 7], [2, 1, 5]]);
movieRentingSystem.search(1);  // return [1, 0, 2], Movies of ID 1 are unrented at shops 1, 0, and 2. Shop 1 is cheapest; shop 0 and 2 are the same price, so order by shop number.
movieRentingSystem.rent(0, 1); // Rent movie 1 from shop 0. Unrented movies at shop 0 are now [2,3].
movieRentingSystem.rent(1, 2); // Rent movie 2 from shop 1. Unrented movies at shop 1 are now [1].
movieRentingSystem.report();   // return [[0, 1], [1, 2]]. Movie 1 from shop 0 is cheapest, followed by movie 2 from shop 1.
movieRentingSystem.drop(1, 2); // Drop off movie 2 at shop 1. Unrented movies at shop 1 are now [1,2].
movieRentingSystem.search(2);  // return [0, 1]. Movies of ID 2 are unrented at shops 0 and 1. Shop 0 is cheapest, followed by shop 1.
</pre>

#### Constraints:
* <code>1 <= n <= 3 * 10<sup>5</sup></code>
* <code>1 <= entries.length <= 10<sup>5</sup></code>
* <code>0 <= shop<sub>i</sub> < n</code>
* <code>1 <= movie<sub>i</sub>, price<sub>i</sub> <= 10<sup>4</sup></code>
* Each shop carries **at most one** copy of a movie <code>movie<sub>i</sub></code>.
* At most <code>10<sup>5</sup></code> calls **in total** will be made to `search`, `rent`, `drop` and `report`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct MovieRentingSystem {
    rented: HashMap<(i32, i32), (bool, i32)>,
    search_heaps: HashMap<i32, BinaryHeap<Reverse<(i32, i32)>>>,
    report_heap: BinaryHeap<Reverse<(i32, i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut rented = HashMap::new();
        let mut search_heaps = HashMap::new();

        for i in 0..entries.len() {
            let (shop, movie, price) = (entries[i][0], entries[i][1], entries[i][2]);
            rented.insert((shop, movie), (false, price));
            search_heaps
                .entry(movie)
                .or_insert(BinaryHeap::new())
                .push(Reverse((price, shop)));
        }

        Self {
            rented: rented,
            search_heaps: search_heaps,
            report_heap: BinaryHeap::new(),
        }
    }

    fn search(&mut self, movie: i32) -> Vec<i32> {
        let mut empty = BinaryHeap::new();
        let mut heap = self.search_heaps.get_mut(&movie).unwrap_or(&mut empty);
        let mut res = vec![];

        while let Some(Reverse((price, shop))) = heap.pop() {
            if !self.rented[&(shop, movie)].0 && shop != *res.last().unwrap_or(&-1) {
                res.push(shop);
            }

            if res.len() == 5 {
                break;
            }
        }

        for &shop in &res {
            heap.push(Reverse((self.rented[&(shop, movie)].1, shop)));
        }

        res
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        self.rented.get_mut(&(shop, movie)).unwrap().0 = true;
        self.report_heap
            .push(Reverse((self.rented[&(shop, movie)].1, shop, movie)));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        self.rented.get_mut(&(shop, movie)).unwrap().0 = false;
        self.search_heaps
            .get_mut(&movie)
            .unwrap()
            .push(Reverse((self.rented[&(shop, movie)].1, shop)));
    }

    fn report(&mut self) -> Vec<Vec<i32>> {
        let mut res = vec![];

        while let Some(Reverse((price, shop, movie))) = self.report_heap.pop() {
            if self.rented[&(shop, movie)].0 && &vec![shop, movie] != res.last().unwrap_or(&vec![])
            {
                res.push(vec![shop, movie]);
            }

            if res.len() == 5 {
                break;
            }
        }

        for i in 0..res.len() {
            let (shop, movie) = (res[i][0], res[i][1]);
            self.report_heap
                .push(Reverse((self.rented[&(shop, movie)].1, shop, movie)));
        }

        res
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
```
