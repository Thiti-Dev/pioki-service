use serde::Serialize;

#[derive(Serialize,Default)]
pub struct StatisticDataOutput{
    pub total_user: i64,
    pub total_created_post: i64,
    pub total_post_kept: i64,
    pub total_post_passed_along: i64
}