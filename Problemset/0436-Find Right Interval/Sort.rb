# @param {Integer[][]} intervals
# @return {Integer[]}
def find_right_interval(intervals)
  starts = []
  ends = []
  i = 0
  ret = [-1] * intervals.size

  (0...intervals.size).each do |j|
    starts.push([intervals[j][0], j])
    ends.push([intervals[j][1], j])
  end
  starts.sort!
  ends.sort!

  (0...ends.size).each do |j|
    i += 1 while i < ends.size && ends[j][0] > starts[i][0]
    break if i >= ends.size

    ret[ends[j][1]] = starts[i][1]
  end

  ret
end
