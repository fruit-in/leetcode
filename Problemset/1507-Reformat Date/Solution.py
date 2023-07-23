class Solution:
    def reformatDate(self, date: str) -> str:
        day, month, year = date.split()
        day = int(day[:-2])
        month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].index(month) + 1

        return "%s-%02d-%02d" % (year, month, day)
