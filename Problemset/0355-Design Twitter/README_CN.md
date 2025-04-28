# 355. 设计推特
设计一个简化版的推特(Twitter)，可以让用户实现发送推文，关注/取消关注其他用户，能够看见关注人（包括自己）的最近 `10` 条推文。

实现 `Twitter` 类：
* `Twitter()` 初始化简易版推特对象
* `void postTweet(int userId, int tweetId)` 根据给定的 `tweetId` 和 `userId` 创建一条新推文。每次调用此函数都会使用一个不同的 `tweetId` 。
* `List<Integer> getNewsFeed(int userId)` 检索当前用户新闻推送中最近  `10` 条推文的 ID 。新闻推送中的每一项都必须是由用户关注的人或者是用户自己发布的推文。推文必须 **按照时间顺序由最近到最远排序** 。
* `void follow(int followerId, int followeeId)` ID 为 `followerId` 的用户开始关注 ID 为 `followeeId` 的用户。
* `void unfollow(int followerId, int followeeId)` ID 为 `followerId` 的用户不再关注 ID 为 `followeeId` 的用户。

#### 示例:
<pre>
<strong>输入:</strong>
["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
[[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
<strong>输出:</strong>
[null, null, [5], null, null, [6, 5], null, [5]]
<strong>解释:</strong>
Twitter twitter = new Twitter();
twitter.postTweet(1, 5); // 用户 1 发送了一条新推文 (用户 id = 1, 推文 id = 5)
twitter.getNewsFeed(1);  // 用户 1 的获取推文应当返回一个列表，其中包含一个 id 为 5 的推文
twitter.follow(1, 2);    // 用户 1 关注了用户 2
twitter.postTweet(2, 6); // 用户 2 发送了一个新推文 (推文 id = 6)
twitter.getNewsFeed(1);  // 用户 1 的获取推文应当返回一个列表，其中包含两个推文，id 分别为 -> [6, 5] 。推文 id 6 应当在推文 id 5 之前，因为它是在 5 之后发送的
twitter.unfollow(1, 2);  // 用户 1 取消关注了用户 2
twitter.getNewsFeed(1);  // 用户 1 获取推文应当返回一个列表，其中包含一个 id 为 5 的推文。因为用户 1 已经不再关注用户 2
</pre>

#### 提示:
* `1 <= userId, followerId, followeeId <= 500`
* <code>0 <= tweetId <= 10<sup>4</sup></code>
* 所有推特的 ID 都互不相同
* `postTweet`、`getNewsFeed`、`follow` 和 `unfollow` 方法最多调用 <code>3 * 10<sup>4</sup></code> 次
* 用户不能关注自己

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter;

struct Twitter {
    time: i32,
    followees: HashMap<i32, HashSet<i32>>,
    user_tweets: HashMap<i32, VecDeque<(Reverse<i32>, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            followees: HashMap::new(),
            user_tweets: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let mut deque = self.user_tweets.entry(user_id).or_insert(VecDeque::new());

        deque.push_front((Reverse(self.time), tweet_id));
        if deque.len() > 10 {
            deque.pop_back();
        }
        self.time += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        let mut ret = vec![];

        for followee in self
            .followees
            .get(&user_id)
            .unwrap_or(&HashSet::new())
            .iter()
            .chain(iter::once(&user_id))
        {
            for &(Reverse(time), tweet_id) in
                self.user_tweets.get(followee).unwrap_or(&VecDeque::new())
            {
                if heap.len() == 10 && heap.peek().unwrap().0 .0 > time {
                    break;
                }
                if heap.len() == 10 {
                    heap.pop();
                }
                heap.push((Reverse(time), tweet_id));
            }
        }

        while let Some((_, tweet_id)) = heap.pop() {
            ret.push(tweet_id);
        }
        ret.reverse();

        ret
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.followees
                .entry(follower_id)
                .or_insert(HashSet::new())
                .insert(followee_id);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.followees
            .get_mut(&follower_id)
            .unwrap_or(&mut HashSet::new())
            .remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
```
