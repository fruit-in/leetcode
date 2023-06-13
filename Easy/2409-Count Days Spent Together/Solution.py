class Solution:
    def countDaysTogether(self, arriveAlice: str, leaveAlice: str, arriveBob: str, leaveBob: str) -> int:
        days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334]
        arriveAliceMM, arriveAliceDD = int(
            arriveAlice[:2]), int(arriveAlice[3:])
        leaveAliceMM, leaveAliceDD = int(leaveAlice[:2]), int(leaveAlice[3:])
        arriveBobMM, arriveBobDD = int(arriveBob[:2]), int(arriveBob[3:])
        leaveBobMM, leaveBobDD = int(leaveBob[:2]), int(leaveBob[3:])
        arriveAliceDay = days[arriveAliceMM - 1] + arriveAliceDD
        leaveAliceDay = days[leaveAliceMM - 1] + leaveAliceDD
        arriveBobDay = days[arriveBobMM - 1] + arriveBobDD
        leaveBobDay = days[leaveBobMM - 1] + leaveBobDD
        ret = min(leaveAliceDay, leaveBobDay) - \
            max(arriveAliceDay, arriveBobDay) + 1

        return max(ret, 0)
