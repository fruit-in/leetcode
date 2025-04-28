# 355. Design Twitter
Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and is able to see the `10` most recent tweets in the user's news feed.

Implement the `Twitter` class:
* `Twitter()` Initializes your twitter object.
* `void postTweet(int userId, int tweetId)` Composes a new tweet with ID `tweetId` by the user `userId`. Each call to this function will be made with a unique `tweetId`.
* `List<Integer> getNewsFeed(int userId)` Retrieves the `10` most recent tweet IDs in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user themself. Tweets must be **ordered from most recent to least recent**.
* `void follow(int followerId, int followeeId)` The user with ID `followerId` started following the user with ID `followeeId`.
* `void unfollow(int followerId, int followeeId)` The user with ID `followerId` started unfollowing the user with ID `followeeId`.

#### Example 1:
<pre>
<strong>Input:</strong>
["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
[[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
<strong>Output:</strong>
[null, null, [5], null, null, [6, 5], null, [5]]
<strong>Explanation:</strong>
Twitter twitter = new Twitter();
twitter.postTweet(1, 5); // User 1 posts a new tweet (id = 5).
twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
twitter.follow(1, 2);    // User 1 follows user 2.
twitter.postTweet(2, 6); // User 2 posts a new tweet (id = 6).
twitter.getNewsFeed(1);  // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
twitter.unfollow(1, 2);  // User 1 unfollows user 2.
twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
</pre>

#### Constraints:
* `1 <= userId, followerId, followeeId <= 500`
* <code>0 <= tweetId <= 10<sup>4</sup></code>
* All the tweets have **unique** IDs.
* At most <code>3 * 10<sup>4</sup></code> calls will be made to `postTweet`, `getNewsFeed`, `follow`, and `unfollow`.
* A user cannot follow himself.

## Solutions (Rust)

### 1. Solution
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
