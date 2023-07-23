# @param {Integer[][]} points
# @return {Integer}
def min_time_to_visit_all_points(points)
    ret = 0

    for i in 0...(points.length - 1)
        x0, y0 = points[i][0], points[i][1]
        x1, y1 = points[i + 1][0], points[i + 1][1]
        ret += [(x0 - x1).abs, (y0 - y1).abs].max
    end

    return ret
end
