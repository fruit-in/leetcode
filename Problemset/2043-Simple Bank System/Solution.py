class Bank:

    def __init__(self, balance: List[int]):
        self.balance = balance

    def transfer(self, account1: int, account2: int, money: int) -> bool:
        from1 = self.withdraw(account1, money)
        to2 = self.deposit(account2, money)

        if from1 and to2:
            return True
        elif from1:
            self.deposit(account1, money)
        elif to2:
            self.withdraw(account2, money)

        return False

    def deposit(self, account: int, money: int) -> bool:
        if account <= len(self.balance):
            self.balance[account - 1] += money

            return True

        return False

    def withdraw(self, account: int, money: int) -> bool:
        if account <= len(self.balance) and self.balance[account - 1] >= money:
            self.balance[account - 1] -= money

            return True

        return False

# Your Bank object will be instantiated and called as such:
# obj = Bank(balance)
# param_1 = obj.transfer(account1,account2,money)
# param_2 = obj.deposit(account,money)
# param_3 = obj.withdraw(account,money)
