use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}};

#[derive(PartialEq, Eq, Clone, Copy)]
struct Tweet {
    timestamp: i32,
    tweet_id: i32
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Twitter {
    tweets: HashMap<i32, VecDeque<Tweet>>,
    follows: HashMap<i32, HashSet<i32>>,
    timestamp: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Self {
            tweets: HashMap::new(),
            follows: HashMap::new(),
            timestamp: 0
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.timestamp += 1;
        self.tweets.entry(user_id).or_default().push_front(Tweet {timestamp: self.timestamp, tweet_id});
        if self.tweets.get(&user_id).unwrap().len() > 10{
            self.tweets.entry(user_id).and_modify(|q| {q.pop_back();});
        }
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let users = self.follows.get(&user_id).unwrap_or(&HashSet::new()) | &HashSet::from([user_id]);
        let mut all_tweets = BinaryHeap::new();
        for user in users {
            all_tweets.extend(self.tweets.get(&user).unwrap_or(&VecDeque::new()).iter().copied());
        }
        let all_tweets: Vec<Tweet> = all_tweets.into_sorted_vec().into_iter().rev().take(10).collect();
        all_tweets.iter().map(|&tweet| tweet.tweet_id).collect()
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if followee_id != follower_id{
            self.follows.entry(follower_id).or_default().insert(followee_id);
        }
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).and_modify(|set|{set.remove(&followee_id);});
    }
}

// /**
//  * Your Twitter object will be instantiated and called as such:
//  * let obj = Twitter::new();
//  * obj.post_tweet(userId, tweetId);
//  * let ret_2: Vec<i32> = obj.get_news_feed(userId);
//  * obj.follow(followerId, followeeId);
//  * obj.unfollow(followerId, followeeId);
//  */