# @param {Integer[]} piles
# @param {Integer} h
# @return {Integer}
def min_eating_speed(piles, h)
  lo = 1
  hi = piles.max

  while lo < hi
    k = (lo + hi) / 2
    hours = piles.map { |x| (x - 1) / k + 1 }.sum

    if hours > h
      lo = k + 1
    else
      hi = k
    end
  end

  lo
end
