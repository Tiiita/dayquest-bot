use std::time::SystemTime;

use dayquest_bot::{config, service};

#[tokio::main]
async fn main() {
    let start_time = SystemTime::now();
    println!("Running DayQuest Discord Bot!");

    let config = config::load();
    service::login(config).await;

    let booting_time = format!(
        "{:.2}",
        SystemTime::now()
            .duration_since(start_time)
            .unwrap()
            .as_secs_f32()
    );

    println!("Started, took {booting_time} s");
}
