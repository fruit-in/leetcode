from queue import SimpleQueue


class Solution:
    def watchedVideosByFriends(self, watchedVideos: List[List[str]], friends: List[List[int]], id: int, level: int) -> List[str]:
        levels = [100] * len(friends)
        levels[id] = 0
        queue = SimpleQueue()
        queue.put((id, 0))
        count = {}
        ret = []

        while not queue.empty():
            i, k = queue.get()

            if k > level:
                break

            for friend in friends[i]:
                if levels[friend] > k + 1:
                    levels[friend] = k + 1
                    queue.put((friend, k + 1))

        for i in range(len(friends)):
            if levels[i] == level:
                for video in watchedVideos[i]:
                    if video not in count:
                        count[video] = 0
                        ret.append(video)
                    count[video] += 1

        return sorted(ret, key=lambda v: (count[v], v))
