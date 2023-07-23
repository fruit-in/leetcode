# 1701. 平均等待时间
有一个餐厅，只有一位厨师。你有一个顾客数组 `customers` ，其中 <code>customers[i] = [arrival<sub>i</sub>, time<sub>i</sub>]</code> ：
* <code>arrival<sub>i</sub></code> 是第 `i` 位顾客到达的时间，到达时间按 **非递减** 顺序排列。
* <code>time<sub>i</sub></code> 是给第 `i` 位顾客做菜需要的时间。

当一位顾客到达时，他将他的订单给厨师，厨师一旦空闲的时候就开始做这位顾客的菜。每位顾客会一直等待到厨师完成他的订单。厨师同时只能做一个人的订单。厨师会严格按照 **订单给他的顺序** 做菜。

请你返回所有顾客需要等待的 **平均** 时间。与标准答案误差在 <code>10<sup>-5</sup></code> 范围以内，都视为正确结果。

#### 示例 1:
<pre>
<strong>输入:</strong> customers = [[1,2],[2,5],[4,3]]
<strong>输出:</strong> 5.00000
<strong>解释:</strong>
1) 第一位顾客在时刻 1 到达，厨师拿到他的订单并在时刻 1 立马开始做菜，并在时刻 3 完成，第一位顾客等待时间为 3 - 1 = 2 。
2) 第二位顾客在时刻 2 到达，厨师在时刻 3 开始为他做菜，并在时刻 8 完成，第二位顾客等待时间为 8 - 2 = 6 。
3) 第三位顾客在时刻 4 到达，厨师在时刻 8 开始为他做菜，并在时刻 11 完成，第三位顾客等待时间为 11 - 4 = 7 。
平均等待时间为 (2 + 6 + 7) / 3 = 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> customers = [[5,2],[5,4],[10,3],[20,1]]
<strong>输出:</strong> 3.25000
<strong>解释:</strong>
1) 第一位顾客在时刻 5 到达，厨师拿到他的订单并在时刻 5 立马开始做菜，并在时刻 7 完成，第一位顾客等待时间为 7 - 5 = 2 。
2) 第二位顾客在时刻 5 到达，厨师在时刻 7 开始为他做菜，并在时刻 11 完成，第二位顾客等待时间为 11 - 5 = 6 。
3) 第三位顾客在时刻 10 到达，厨师在时刻 11 开始为他做菜，并在时刻 14 完成，第三位顾客等待时间为 14 - 10 = 4 。
4) 第四位顾客在时刻 20 到达，厨师拿到他的订单并在时刻 20 立马开始做菜，并在时刻 21 完成，第四位顾客等待时间为 21 - 20 = 1 。
平均等待时间为 (2 + 6 + 4 + 1) / 4 = 3.25 。
</pre>

#### 提示:
* <code>1 <= customers.length <= 10<sup>5</sup></code>
* <code>1 <= arrival<sub>i</sub>, time<sub>i</sub> <= 10<sup>4</sup></code>
* <code>arrival<sub>i</sub> <= arrival<sub>i+1</sub></code>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} customers
# @return {Float}
def average_waiting_time(customers)
  time = 0
  sum = 0

  customers.each do |customer|
    if customer[0] >= time
      sum += customer[1]
      time = customer.sum
    else
      sum += time + customer[1] - customer[0]
      time += customer[1]
    end
  end

  1.0 * sum / customers.size
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut time = 0u64;
        let mut sum = 0u64;

        for customer in &customers {
            if customer[0] as u64 >= time {
                sum += customer[1] as u64;
                time = (customer[0] + customer[1]) as u64;
            } else {
                sum += time + (customer[1] - customer[0]) as u64;
                time += customer[1] as u64;
            }
        }

        sum as f64 / customers.len() as f64
    }
}
```
