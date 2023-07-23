# @param {Integer} n
# @return {Integer}
def number_of_matches(n)
  if n == 1
    0
  elsif n.even?
    n / 2 + number_of_matches(n / 2)
  else
    (n - 1) / 2 + number_of_matches((n - 1) / 2 + 1)
  end
end
