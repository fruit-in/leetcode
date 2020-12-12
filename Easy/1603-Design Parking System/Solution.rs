struct ParkingSystem {
    slots: [i32; 3],
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {slots: [big, medium, small]}
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots[car_type as usize - 1] > 0 {
            self.slots[car_type as usize - 1] -= 1;

            true
        } else {
            false
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
