# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @param {Integer} d
# @param {Integer} e
# @param {Integer} f
# @param {Integer} g
# @param {Integer} h
# @return {Integer}
def compute_area(a, b, c, d, e, f, g, h)
    area0 = (c - a) * (d - b)
    area1 = (g - e) * (h - f)
    area2 = [[c, g].min - [a, e].max, 0].max * [[d, h].min - [b, f].max, 0].max

    area0 + area1 - area2
end
