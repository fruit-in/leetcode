# @param {Integer} num
# @return {Boolean}
def check_perfect_number(num)
    return false if num <= 1

    sum = 1
    i = 2

    while i * i < num
        sum += i + num / i if num % i == 0
        i += 1
    end

    sum += i if i * i == num

    return sum == num
end
