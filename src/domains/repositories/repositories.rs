use crate::repository::{friends::FriendRepository, posts::PostRepository, statistic::StatisticRepository, users::UserRepository};

pub struct Repositories{
    pub post_repository: PostRepository,
    pub friend_repository: FriendRepository,
    pub user_repository: UserRepository,
    pub statistic_repository: StatisticRepository
}