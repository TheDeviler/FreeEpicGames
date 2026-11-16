use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use chrono::{Local, Timelike};
use colored::Colorize;
use crossterm::event::{Event, KeyCode, read};
use serde_json::Value;

const FREE_GAMES_URL: &str =
    "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions";
const EPIC_GAME_URL: &str = "https://store.epicgames.com/en-US/p/";

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let now = Local::now();
    let username = whoami::username();

    println!(
        "{}\n",
        "▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄ Developed by TheDevIsHere ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀".green()
    );
    sleep(Duration::from_secs(1));

    let greeting = if now.hour() >= 18 {
        "Good evening"
    } else {
        "Good morning"
    };
    println!(
        "{greeting}: {}!\nProgram start time: {}\n",
        username.blue(),
        now.format("%H:%M").to_string().green()
    );
    println!("Now looking for free games...");
    sleep(Duration::from_secs(1));

    let data: Value = reqwest::blocking::get(FREE_GAMES_URL)?
        .error_for_status()?
        .json()?;
    let games = data["data"]["Catalog"]["searchStore"]["elements"]
        .as_array()
        .ok_or("Epic Games returned an unexpected response")?;

    let mut free_games = 0;
    for game in games {
        if !is_free(game) {
            continue;
        }

        let title = game["title"].as_str().unwrap_or("Unknown game");
        println!("{} {}", "Free game:".green(), title.red());
        free_games += 1;

        if let Some(slug) = game["catalogNs"]["mappings"]
            .get(0)
            .and_then(|mapping| mapping["pageSlug"].as_str())
        {
            webbrowser::open(&format!("{EPIC_GAME_URL}{slug}"))?;
        }
    }

    if free_games == 0 {
        println!("No free games today!");
        sleep(Duration::from_secs(3));
        return Ok(());
    }

    println!(
        "\n{}\n",
        "▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄ Free games opened in your browser ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀".green()
    );
    println!("{}", "Press Backspace to close.".bold());

    loop {
        if let Event::Key(key_event) = read()? {
            if key_event.code == KeyCode::Backspace {
                break;
            }
        }
    }

    Ok(())
}

fn is_free(game: &Value) -> bool {
    game["promotions"]["promotionalOffers"][0]["promotionalOffers"][0]["discountSetting"]
        ["discountPercentage"]
        .as_i64()
        == Some(0)
}
