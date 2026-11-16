use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, Timelike};
use colored::Colorize;
use crossterm::event::{read, Event};

fn main() 
{
    let local_now:i32 = Local::now().hour() as i32;
    let local_program = Local::now().format("%H:%M");
    let usr: String = whoami::username();
    let url = "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions";
    let response = reqwest::blocking::get(url).unwrap();
    let data: serde_json::Value = response.json().unwrap();
    println!("{}\n", "▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄ Developed by TheDevIsHere ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀".green());
    sleep(Duration::from_secs(1));

    if local_now >= 18
    {
        println!("Good evening: {}!\nProgram Start time: {} ", usr.blue(), (local_program).to_string().green());
    }
    else 
    {
        println!("Good morning: {}!\nProgram Start time: {} ", usr.blue(), (local_program).to_string().green());
    }
    println!("Now looking for free games...");
    sleep(Duration::from_secs(1));

    let games = &data["data"]["Catalog"]["searchStore"]["elements"];

    let mut i: i32 = 0;
    for game in games.as_array().unwrap() 
    {
        
        let title: &str = game["title"].as_str().unwrap_or("idk");
        let slug = game["catalogNs"]["mappings"].get(0).and_then(|m| m["pageSlug"].as_str());


        let discount = game["promotions"]
            ["promotionalOffers"][0]
            ["promotionalOffers"][0]
            ["discountSetting"]["discountPercentage"]
            .as_i64();

        if discount == Some(0) 
        {
            println!("{} {}", "Free games:".green(), title.red());
            if let Some(slug) = slug 
            {
                let url = format!("https://store.epicgames.com/en-US/p/{}", slug);
                
                i+=1;
                if let Err(e) = webbrowser::open(&url) {
                    println!("Browser error: {}", e);
                }
            }
        }
    }
    if i==0
    {
        println!("No free games today!");
        println!("Closing now.....");
        sleep(Duration::from_secs(3));
        return; 
    }
    println!("\n{}\n", "▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀".green());
    println!("{}", "Press backspace to close".bold());
    loop {
        if let Ok(Event::Key(key_event)) = read() {
            if key_event.code.is_backspace() 
            {
                break;                
            }
        }
    }
}
