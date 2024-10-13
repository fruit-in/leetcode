class Solution:
    def mostPopularCreator(self, creators: List[str], ids: List[str], views: List[int]) -> List[List[str]]:
        popularities = {}
        mostviewvideo = {}
        maxpopularity = 0
        answer = set()

        for i in range(len(ids)):
            video, mostview = mostviewvideo.get(creators[i], ("", -1))
            popularities[creators[i]] = popularities.get(
                creators[i], 0) + views[i]

            if views[i] > mostview or (views[i] == mostview and ids[i] < video):
                mostviewvideo[creators[i]] = (ids[i], views[i])
            if popularities[creators[i]] > maxpopularity:
                maxpopularity = popularities[creators[i]]
                answer.clear()
            if popularities[creators[i]] == maxpopularity:
                answer.add((creators[i], mostviewvideo[creators[i]][0]))

        return [[c, d] for c, d in answer]
