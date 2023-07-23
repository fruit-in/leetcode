class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        bulls = 0

        scows = [0] * 10
        gcows = [0] * 10

        for sch, gch in zip(secret, guess):
            if sch == gch:
                bulls += 1
            else:
                scows[int(sch)] += 1
                gcows[int(gch)] += 1

        cows = sum(min(scow, gcow) for scow, gcow in zip(scows, gcows))

        return "%dA%dB" % (bulls, cows)
