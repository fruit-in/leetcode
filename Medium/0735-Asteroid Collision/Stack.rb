# @param {Integer[]} asteroids
# @return {Integer[]}
def asteroid_collision(asteroids)
  ret = []

  asteroids.each do |asteroid|
    if asteroid > 0
      ret.push(asteroid)
    else
      ret.pop while !ret.empty? && ret[-1] > 0 && ret[-1] < -asteroid
      if !ret.empty? && ret[-1] > 0 && ret[-1] == -asteroid
        ret.pop
      elsif ret.empty? || ret[-1] < 0
        ret.push(asteroid)
      end
    end
  end

  ret
end
