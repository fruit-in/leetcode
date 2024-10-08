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
