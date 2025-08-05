use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

use chrono::{NaiveTime, DateTime, Datelike, Months, Utc };
use chrono_tz::Tz;
use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;

const GH_TIME_SPEED_MULTIPLIER: i64 = 14;
const JAN_FIRST_2000: i64 = 946684800;
const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0)
    .expect("Unable to get beginning of day");

fn get_gh_timestamps(irl_gh_origin: i64) -> (i64, i64, i64) {
    let current_time: i64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Can't get time")
    .as_secs() as i64;

    let gh_current_timestamp: i64 = (
        (current_time - irl_gh_origin) * GH_TIME_SPEED_MULTIPLIER
    ) + JAN_FIRST_2000;
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
    let irl_timestamp_next_zday: i64 = current_time + (
        gh_seconds_until_next_zday / GH_TIME_SPEED_MULTIPLIER
    );
    (gh_current_timestamp, irl_timestamp_next_zday - current_time, irl_timestamp_next_zday)
}

#[tokio::main]
async fn main() {
    let irl_gh_origin: i64 = match env::var("IRL_GH_ORIGIN") {
        Ok(val) => val.parse::<i64>()
            .expect(format!("IRL_GH_ORIGIN is present but not an i64. Got {}", val).as_str()),
        Err(_) => 1739039409, // Server clock origin as of 2025/07/28
    };
    let alert_threshold: i64 = match env::var("ALERT_THRESHOLD") {
        Ok(val) => val.parse::<i64>()
            .expect(format!("ALERT_THRESHOLD is present but not an i64. Got {}", val).as_str()),
        Err(_) => 86400, // 24 hours
    };
    let alert_enabled: bool = match env::var("ALERT_ENABLED") {
        Ok(val) => val.parse::<bool>()
            .expect(format!("ALERT_ENABLED is present but not a bool. Got {}", val).as_str()),
        Err(_) => false,
    };
    let tz_str = match env::var("TZ") {
        Ok(val) => val,
        Err(_) => String::from("UTC"),
    };
    let tz: Tz = match tz_str.parse::<Tz>() {
        Ok(tz) => tz,
        Err(_) => {
            eprintln!("Invalid timezone: {}. Defaulting to UTC", tz_str);
            String::from("UTC").parse::<Tz>().expect("Unable to get UTC timezone")
        }
    };

    let timestamps: (i64, i64, i64) = get_gh_timestamps(irl_gh_origin);
    let current_server_clock: i64 = timestamps.0;
    let seconds_until: i64 = timestamps.1;
    let irl_zday_timestamp: i64 = timestamps.2;
    let message: String = format!("Zero-day is near: <t:{}:F>", irl_zday_timestamp);

    let zerday_datetime: DateTime<Tz> = DateTime::from_timestamp(irl_zday_timestamp, 0)
        .expect("Unable to convert zerday timestamp").with_timezone(&tz).into();
    let server_datetime: DateTime<Utc> = DateTime::from_timestamp(current_server_clock, 0)
        .expect("Unable to convert current server clock").into();
    println!("{}", message);
    println!();
    println!("(local time) Zero-day: {}", zerday_datetime.format("%Y/%m/%d %H:%M:%S"));
    println!("In-game time:          {}", server_datetime.format("%Y/%m/%d %H:%M:%S"));

    // If zero-day is near, message the server
    let would_alert: bool = seconds_until < alert_threshold;
    if alert_enabled && would_alert {
        // Login with a bot token from the environment
        let webhook_url: String = env::var("WEBHOOK")
            .expect("Expected a token in the environment");

        // You don't need a token when you are only dealing with webhooks.
        let http: Http = Http::new("");
        let webhook: Webhook = Webhook::from_url(&http, webhook_url.as_str())
            .await
            .expect("Replace the webhook with your own");

        let builder: ExecuteWebhook = ExecuteWebhook::new().content(message);
        webhook.execute(&http, false, builder).await
            .expect("Could not execute webhook.");
    } else {
        println!("Would alert: {:?}", would_alert);
    }
}
