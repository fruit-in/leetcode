# @param {Integer} x
# @param {Integer} y
# @param {Integer} z
# @return {Boolean}
def can_measure_water(x, y, z)
  x + y >= z && (x + y == 0 || z % x.gcd(y) == 0)
end
