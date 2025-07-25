use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

use chrono::{NaiveTime, DateTime, Datelike, Months, Utc};
use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;

const GH_TIME_SPEED_MULTIPLIER: i64 = 14;
const HRS_48: i64 = 86400 * 2;
const JAN_FIRST_2000: i64 = 946684800;
const IRL_GH_ORIGIN: i64 = 1739039409; // The IRL timestamp when GH Jan 1st, 2000 began.
const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0)
.expect("Unable to get beginning of day");

fn get_seconds_until_zday() -> (i64, i64) {
    let current_time: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't get time")
        .as_secs() as i64;

    let gh_current_timestamp: i64 = ((current_time - IRL_GH_ORIGIN) * GH_TIME_SPEED_MULTIPLIER) + JAN_FIRST_2000;
    let gh_current_datetime: DateTime<Utc> = DateTime::from_timestamp(gh_current_timestamp, 0)
        .expect("Cannot convert to DateTime<Utc>");

    let gh_beginning_of_day: DateTime<Utc> = gh_current_datetime.with_time(DAY_BEGIN).unwrap();
    let gh_beginning_of_month: DateTime<Utc> = gh_beginning_of_day.with_day(1)
        .expect("Unable to get start of month");

    let mut delta_months: u32 = 1;
    if gh_beginning_of_month.month() % 2 == 0 {
        delta_months = 2;
    }
    let gh_zday_month: DateTime<Utc> = gh_beginning_of_month
        .checked_add_months(Months::new(delta_months))
        .expect("Unable to get next month");

    let gh_seconds_until_next_zday: i64 = gh_zday_month.timestamp() - gh_current_timestamp;
    let irl_timestamp_next_zday: i64 = current_time + (gh_seconds_until_next_zday / GH_TIME_SPEED_MULTIPLIER);
    (irl_timestamp_next_zday - current_time, irl_timestamp_next_zday)
}

#[tokio::main]
async fn main() {
    let timestamps: (i64, i64) = get_seconds_until_zday();
    let seconds_until: i64 = timestamps.0;
    let irl_zday_timestamp: i64 = timestamps.1;

    // // If zero-day is near, message the server
    if seconds_until < HRS_48 {
        // Login with a bot token from the environment
        let webhook_url = env::var("WEBHOOK").expect("Expected a token in the environment");

        // You don't need a token when you are only dealing with webhooks.
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, webhook_url.as_str())
            .await
            .expect("Replace the webhook with your own");

        let message = format!("Zero-day is near: <t:{}:F>", irl_zday_timestamp);
        let builder = ExecuteWebhook::new().content(message);
        webhook.execute(&http, false, builder).await.expect("Could not execute webhook.");
    }

    println!("<t:{}:F>", irl_zday_timestamp);
}
