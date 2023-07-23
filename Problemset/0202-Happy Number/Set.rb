# @param {Integer} n
# @return {Boolean}
def is_happy(n)
    set = Set.new

    while not set.include?(n)
        set.add(n)
        new_n = 0
        while n > 0
            new_n += (n % 10) ** 2
            n /= 10
        end
        n = new_n
    end

    return n == 1
end
