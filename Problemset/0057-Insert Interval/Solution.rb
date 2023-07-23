# @param {Integer[][]} intervals
# @param {Integer[]} new_interval
# @return {Integer[][]}
def insert(intervals, new_interval)
  flag = true
  ret = []

  intervals.each do |interval|
    if interval[0] > new_interval[1]
      ret.push(new_interval) if flag
      ret.push(interval)
      flag = false
    elsif interval[1] < new_interval[0]
      ret.push(interval)
    else
      new_interval[0] = [new_interval[0], interval[0]].min
      new_interval[1] = [new_interval[1], interval[1]].max
    end
  end
  ret.push(new_interval) if flag

  ret
end
