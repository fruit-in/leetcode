# @param {Integer[][]} points
# @return {Integer}
def max_width_of_vertical_area(points)
  ret = 0

  points.sort_by! { |p| p[0] }

  (1...points.length).each do |i|
    ret = [ret, points[i][0] - points[i - 1][0]].max
  end

  ret
end
