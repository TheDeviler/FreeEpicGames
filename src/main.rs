use chrono::{Local, Timelike};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let local_now: i32 = Local::now().hour() as i32;
    let local_program = Local::now().format("%H:%M");
    let usr: String = whoami::username();
    let url = "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=it-IT&country=IT&allowCountries=IT";
    let response = reqwest::blocking::get(url).unwrap();
    let data: serde_json::Value = response.json().unwrap();
    println!("▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄ Developed by TheDevIsHere ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀");
    sleep(Duration::from_secs(1));

    if local_now >= 18 {
        println!(
            "Good evening: {}!\nProgram Start time: {} ",
            usr, local_program
        );
    } else {
        println!(
            "Good morning: {}!\nProgram Start time: {} ",
            usr, local_program
        );
    }

    let games = &data["data"]["Catalog"]["searchStore"]["elements"];

    for game in games.as_array().unwrap() {
        let title: &str = game["title"].as_str().unwrap_or("idk");
        let slug = game["catalogNs"]["mappings"]
            .get(0)
            .and_then(|m| m["pageSlug"].as_str());

        let discount = game["promotions"]
            ["promotionalOffers"][0]
            ["promotionalOffers"][0]
            ["discountSetting"]["discountPercentage"]
            .as_i64();

        if discount == Some(0) {
            println!("Free games: {}", title);

            if let Some(slug) = slug {
                let url = format!("https://store.epicgames.com/en-US/p/{}", slug);

                if let Err(e) = webbrowser::open(&url) {
                    println!("Browser error: {}", e);
                }
            }
        }
    }
}
