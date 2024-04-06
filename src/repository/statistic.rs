use std::{rc::Rc, sync::{Arc, Mutex}, thread, time::Duration};
use diesel::{dsl::count_star, prelude::*, sql_query};
use tokio::task;
use futures::{future, FutureExt};

use crate::domains::outputs::main::StatisticDataOutput;
#[derive(Clone)]
pub struct StatisticRepository{
    pub db_pool: Rc<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>>,
}

pub async fn my_async_function() -> i32 {
    thread::sleep(Duration::from_secs(5));
    // perform some asynchronous operations
    42 // return a value
}

impl StatisticRepository{
   fn get_total_user(&self) -> i64{
        use crate::schema::users::dsl::*;

        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = users.select(count_star()).first::<i64>(connection).expect("failed getting total users by get_total_users");

        count
    }

   fn get_total_post(&self) -> i64{
        use crate::schema::posts::dsl::*;

        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = posts.select(count_star()).first::<i64>(connection).expect("failed getting count by get_total_posts");

        count
    }

    fn get_total_post_kept(&self) -> i64{
        use crate::schema::post_keepers::dsl::*;

        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = post_keepers.select(count_star()).first::<i64>(connection).expect("failed getting count by get_total_posts_kept");

        count
    }

    fn get_total_pass_along(&self) -> i64{
        use crate::schema::keep_and_pass_along_logs::dsl::*;

        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = keep_and_pass_along_logs.select(count_star()).filter(is_kept.eq(true)).first::<i64>(connection).expect("failed getting count by get_total_posts_kept");

        count
    }

    pub fn get_statistic_data(&self) -> StatisticDataOutput{
        // TODO: make them pararell by tokio thread <deadpool/bbs needed>
        let total_user_count = self.get_total_user();
        let total_post_count = self.get_total_post();
        let total_post_kept = self.get_total_post_kept();
        let total_pass_along = self.get_total_pass_along();

        StatisticDataOutput{
            total_user: total_user_count,
            total_created_post: total_post_count,
            total_post_kept,
            total_post_passed_along:total_pass_along
        }
    }
}