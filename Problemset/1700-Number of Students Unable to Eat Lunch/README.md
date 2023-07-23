# 1700. Number of Students Unable to Eat Lunch
The school cafeteria offers circular and square sandwiches at lunch break, referred to by numbers `0` and `1` respectively. All students stand in a queue. Each student either prefers square or circular sandwiches.

The number of sandwiches in the cafeteria is equal to the number of students. The sandwiches are placed in a **stack**. At each step:
* If the student at the front of the queue **prefers** the sandwich on the top of the stack, they will **take it** and leave the queue.
* Otherwise, they will **leave it** and go to the queue's end.

This continues until none of the queue students want to take the top sandwich and are thus unable to eat.

You are given two integer arrays `students` and `sandwiches` where `sandwiches[i]` is the type of the <code>i<sup>th</sup></code> sandwich in the stack (`i = 0` is the top of the stack) and `students[j]` is the preference of the <code>j<sup>th</sup></code> student in the initial queue (`j = 0` is the front of the queue). Return *the number of students that are unable to eat*.

#### Example 1:
<pre>
<strong>Input:</strong> students = [1,1,0,0], sandwiches = [0,1,0,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
- Front student leaves the top sandwich and returns to the end of the line making students = [1,0,0,1].
- Front student leaves the top sandwich and returns to the end of the line making students = [0,0,1,1].
- Front student takes the top sandwich and leaves the line making students = [0,1,1] and sandwiches = [1,0,1].
- Front student leaves the top sandwich and returns to the end of the line making students = [1,1,0].
- Front student takes the top sandwich and leaves the line making students = [1,0] and sandwiches = [0,1].
- Front student leaves the top sandwich and returns to the end of the line making students = [0,1].
- Front student takes the top sandwich and leaves the line making students = [1] and sandwiches = [1].
- Front student takes the top sandwich and leaves the line making students = [] and sandwiches = [].
Hence all students are able to eat.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `1 <= students.length, sandwiches.length <= 100`
* `students.length == sandwiches.length`
* `sandwiches[i]` is `0` or `1`.
* `students[i]` is `0` or `1`.

## Solutions (Ruby)

### 1. Simulation
```Ruby
# @param {Integer[]} students
# @param {Integer[]} sandwiches
# @return {Integer}
def count_students(students, sandwiches)
  students = Containers::Queue.new(students)
  count = [0, 0]
  i = 0

  students.each do |prefer|
    count[prefer] += 1
  end

  until students.empty?
    prefer = students.pop
    if prefer == sandwiches[i]
      i += 1
      count[prefer] -= 1
    elsif count[0] == 0 || count[1] == 0
      students.push(prefer)
      break
    else
      students.push(prefer)
    end
  end

  students.size
end
```

## Solutions (Rust)

### 1. Simulation
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students.into_iter().collect::<VecDeque<_>>();
        let mut count = [0, 0];
        let mut i = 0;

        for &prefer in &students {
            count[prefer as usize] += 1;
        }

        while let Some(prefer) = students.pop_front() {
            if prefer == sandwiches[i] {
                i += 1;
                count[prefer as usize] -= 1;
            } else if count[0] == 0 || count[1] == 0 {
                students.push_back(prefer);
                break;
            } else {
                students.push_back(prefer);
            }
        }

        students.len() as i32
    }
}
```
