# @param {Integer[][]} points
# @return {Integer}
def find_min_arrow_shots(points)
  points.sort_by! { |p| p[1] }
  x = points[0][1]
  ret = 1

  (1...points.size).each do |i|
    if points[i][0] > x
      x = points[i][1]
      ret += 1
    end
  end

  ret
end
