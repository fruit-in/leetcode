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
