# @param {Integer[][]} accounts
# @return {Integer}
def maximum_wealth(accounts)
  accounts.map { |account| account.sum }.max
end
