# @param {Integer} n
# @param {Integer} k
# @return {String}
def get_happy_string(n, k)
  happy = %w[a b c]

  (2..n).each do |_i|
    happy_ = []

    happy.each do |s|
      'abc'.chars.each do |c|
        happy_.push(s + c) if s[-1] != c
      end
    end

    happy = happy_
  end

  k > happy.size ? '' : happy[k - 1]
end
