# 1845. Seat Reservation Manager
Design a system that manages the reservation state of `n` seats that are numbered from `1` to `n`.

Implement the `SeatManager` class:
* `SeatManager(int n)` Initializes a `SeatManager` object that will manage `n` seats numbered from `1` to `n`. All seats are initially available.
* `int reserve()` Fetches the **smallest-numbered** unreserved seat, reserves it, and returns its number.
* `void unreserve(int seatNumber)` Unreserves the seat with the given seatNumber.

#### Example 1:
<pre>
<strong>Input:</strong>
["SeatManager", "reserve", "reserve", "unreserve", "reserve", "reserve", "reserve", "reserve", "unreserve"]
[[5], [], [], [2], [], [], [], [], [5]]
<strong>Output:</strong>
[null, 1, 2, null, 2, 3, 4, 5, null]
<strong>Explanation:</strong>
SeatManager seatManager = new SeatManager(5); // Initializes a SeatManager with 5 seats.
seatManager.reserve();    // All seats are available, so return the lowest numbered seat, which is 1.
seatManager.reserve();    // The available seats are [2,3,4,5], so return the lowest of them, which is 2.
seatManager.unreserve(2); // Unreserve seat 2, so now the available seats are [2,3,4,5].
seatManager.reserve();    // The available seats are [2,3,4,5], so return the lowest of them, which is 2.
seatManager.reserve();    // The available seats are [3,4,5], so return the lowest of them, which is 3.
seatManager.reserve();    // The available seats are [4,5], so return the lowest of them, which is 4.
seatManager.reserve();    // The only available seat is seat 5, so return 5.
seatManager.unreserve(5); // Unreserve seat 5, so now the available seats are [5].
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= seatNumber <= n`
* For each call to `reserve`, it is guaranteed that there will be at least one unreserved seat.
* For each call to `unreserve`, it is guaranteed that `seatNumber` will be reserved.
* At most <code>10<sup>5</sup></code> calls **in total** will be made to `reserve` and `unreserve`.

## Solutions (Python)

### 1. Solution
```Python
class SeatManager:

    def __init__(self, n: int):
        self.seats = list(range(1, n + 1))
        heapq.heapify(self.seats)

    def reserve(self) -> int:
        return heapq.heappop(self.seats)

    def unreserve(self, seatNumber: int) -> None:
        heapq.heappush(self.seats, seatNumber)

# Your SeatManager object will be instantiated and called as such:
# obj = SeatManager(n)
# param_1 = obj.reserve()
# obj.unreserve(seatNumber)
```
