class Solution:
    def numberOfRounds(self, loginTime: str, logoutTime: str) -> int:
        loginhour = int(loginTime[:2])
        loginmin = int(loginTime[3:])
        logouthour = int(logoutTime[:2]) + \
            (24 if logoutTime < loginTime else 0)
        logoutmin = int(logoutTime[3:])
        ret = (logouthour + 1 - loginhour) * 4

        if loginmin > 45:
            ret -= 4
        elif loginmin > 30:
            ret -= 3
        elif loginmin > 15:
            ret -= 2
        elif loginmin > 0:
            ret -= 1
        if logoutmin < 15:
            ret -= 4
        elif logoutmin < 30:
            ret -= 3
        elif logoutmin < 45:
            ret -= 2
        else:
            ret -= 1

        return max(ret, 0)
