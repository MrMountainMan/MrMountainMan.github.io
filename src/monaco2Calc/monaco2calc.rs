use std::{collections::HashMap, ptr::null};
use regex::Regex;
use dioxus::{html::input, prelude::*};

use dioxus_logger::tracing;


pub fn Monaco2CalcMain() -> Element
{
    let coins_per_level: Signal<HashMap<&str, u16>> = use_signal(|| HashMap::from([
        ("yacht_club", 130),
        ("hotel", 169),
        ("bona_fide_sanitation", 270),
        ("cocktail_party", 317),
        ("prison", 231),
        ("museum", 296),
        ("shipyard", 362),
        ("data_center", 300),
        ("night_club", 399),
        ("opera", 294),
        ("bank", 403),
        ("palace", 337), //unconfirmed
        ("safehouse", 310), //unconfirmed
        ("catacombs", 525), //unconfirmed
        ("casino", 464), //unconfirmed
        ("bonhomme", 589), //unconfirmed
        ("petit_bank_optional", 315), //unconfirmed
        ("museum_optional", 271), //unconfirmed
        ("office_optional", 268), //unconfirmed
        ("art_gallery_optional", 274), //unconfirmed
    ]));

    let mut level: Signal<String> = use_signal(|| "yacht_club".to_string());
    let mut players: Signal<u16> = use_signal(|| 8);
    let mut coins: Signal<u16> = use_signal(|| 0);

    let mut hours: Signal<u16> = use_signal(|| 0);
    let mut minutes: Signal<u16> = use_signal(|| 0);
    let mut seconds: Signal<u16> = use_signal(|| 0);
    let mut milliseconds: Signal<u16> = use_signal(|| 0);

    rsx!
    {
        //change the icon and the website title
        document::Link {rel: "icon", href: asset!("./assets/monaco2.ico")}
        document::Title{"Monaco 2 Score Calculator"}
        div
        {   
            //display header
            h1 {"Monaco 2 Score Calculator"}
            br {}
            {LevelSelector(level)}
            br {}
            "Level selected is: {level}"
            br {}
            br {}
            {PlayerSelector(players)}
            br {}
            "Player multiplier is: {players}"
            br {}
            br {}
            {CoinsCollected(coins, level, coins_per_level)}"/{coins_per_level()[&level() as &str]}"
            br {}
            "Coins collected: {coins}"
            br {}
            br {}
            {CompletionTime(hours, minutes, seconds, milliseconds)}
            br {}
            br {}
            "Final score: {CalculateScore(level, players, coins, hours, minutes, seconds, milliseconds)}"



        }


    }
}

pub fn LevelSelector(mut level: Signal<String>) -> Element
{
    rsx!
    {
        label { r#for: "level_name", "Level: " }
            select
            {
                name: "level",
                id: "level_name",
                value: level,
                oninput: move |event| { *level.write() = event.value(); },
                option { value: "yacht_club", "Yacht Club"}
                option { value: "hotel", "Hotel"}
                option { value: "bona_fide_sanitation", "Bona Fide Sanitation"}
                option { value: "cocktail_party", "Cocktail Party"}
                option { value: "prison", "Prison"}
                option { value: "museum", "Museum"}
                option { value: "shipyard", "Shipyard"}
                option { value: "data_center", "Data Center"}
                option { value: "night_club", "Night Club"}
                option { value: "opera", "Opera"}
                option { value: "bank", "Bank"}
                option { value: "palace", "Palace"}
                option { value: "safehouse", "Safehouse"}
                option { value: "catacombs", "Catacombs"}
                option { value: "casino", "Casino"}
                option { value: "bonhomme", "Bonhomme"}
                option { value: "petit_bank_optional", "Petit Bank (Optional)"}
                option { value: "museum_optional", "Museum (Optional)"}
                option { value: "office_optional", "Office (Optional)"}
                option { value: "art_gallery_optional", "Art Gallery (Optional)"}
            }
    }
}

pub fn PlayerSelector(mut players: Signal<u16>) -> Element
{

    rsx!
    {
        label { r#for: "num_players", "Number of Players: " }
        select
        {
            name: "players",
            id: "num_players",
            onchange: move |event| { *players.write() = event.value().parse().unwrap(); },
            option { value: 8, "1"}
            option { value: 6, "2"}
            option { value: 5, "3"}
            option { value: 4, "4"}
            
        }
    }
}

pub fn CoinsCollected(mut coins: Signal<u16>, level: Signal<String>, coins_per_level: Signal<HashMap<&str, u16>>) -> Element
{
    rsx!
    {
        label { r#for: "coins_collected", "Coins Collected: " }
        input {
            id: "coins_collected",
            r#type: "number",
            min: 0,
            max: coins_per_level()[&level() as &str],
            value: coins,
            oninput: move |event| { CheckValidInput(event, coins, coins_per_level()[&level() as &str]); },
            onpaste: move |event| { event.prevent_default(); },
            onkeypress: move |event| { PreventNonNumericalInput(event); },
            onkeyup: move |_| { CheckOverflow(coins, coins_per_level()[&level() as &str]) },
        }
    }

}


pub fn CompletionTime(hours: Signal<u16>, minutes: Signal<u16>, seconds: Signal<u16>, milliseconds: Signal<u16>) -> Element
{
    rsx!
    {
        "Time to complete: "
            input {
                id: "clear_time_hours",
                r#type: "number",
                min: 0,
                max: 99,
                value: hours,
                oninput: move |event| { CheckValidInput(event, hours, 99); },
                onpaste: move |event| { event.prevent_default(); },
                onkeypress: move |event| { PreventNonNumericalInput(event); },
                onkeyup: move |event| { CheckOverflow(hours, 99) },
            }
            ":"
            input {
                id: "clear_time_minutes",
                r#type: "number",
                min: 0,
                max: 59,
                value: minutes,
                oninput: move |event| { CheckValidInput(event, minutes, 59); },
                onpaste: move |event| { event.prevent_default(); },
                onkeypress: move |event| { PreventNonNumericalInput(event); },
                onkeyup: move |event| { CheckOverflow(minutes, 59) },
            }
            ":"
            input {
                id: "clear_time_seconds",
                r#type: "number",
                min: 0,
                max: 59,
                value: seconds,
                oninput: move |event| { CheckValidInput(event, seconds, 59); },
                onpaste: move |event| { event.prevent_default(); },
                onkeypress: move |event| { PreventNonNumericalInput(event); },
                onkeyup: move |event| { CheckOverflow(seconds, 59) },
            }
            "."
            input {
                id: "clear_time_milliseconds",
                r#type: "number",
                min: 0,
                max: 999,
                value: milliseconds,
                oninput: move |event| { CheckValidInput(event, milliseconds, 999); },
                onpaste: move |event| { event.prevent_default(); },
                onkeypress: move |event| { PreventNonNumericalInput(event); },
                onkeyup: move |event| { CheckOverflow(milliseconds, 999) },
            }
    }

}

pub fn PreventNonNumericalInput(event: Event<KeyboardData>)
{
    let reg = Regex::new(r"[0-9]+").unwrap();
    if !reg.is_match(&(event.key().to_string()))
    {
        event.prevent_default();
        return;
    }
}

pub fn CheckOverflow(mut value: Signal<u16>, max_value: u16)
{
    if value() > max_value
    {
        *value.write() = max_value;
    }
}

pub fn CheckValidInput(event: Event<FormData>, mut value: Signal<u16>, max_value: u16)
{
    if event.value().is_empty()
    {
        *value.write() = 0;
        return;
    }

    if event.value().parse::<u16>().is_ok() && event.value().parse::<u16>().unwrap() <= max_value
    {
        *value.write() = event.value().parse().unwrap();
        return;
    }

    *value.write() = value();

}

pub fn CalculateScore(level: Signal<String>, players: Signal<u16>, coins: Signal<u16>, hours: Signal<u16>, minutes: Signal<u16>, seconds: Signal<u16>, milliseconds: Signal<u16>) -> f64
{
    /*
    let mut f_hours: f32 = 0.0;
    let mut f_minutes: f32 = 0.0;
    let mut f_seconds: f32 = 0.0;
    let mut f_milliseconds: f32 = 0.0;

    if !hours().is_empty()
    {
        f_hours = hours() as f32;
    }

    if !minutes().is_empty()
    {
        f_minutes = minutes() as f32;
    }

    if !seconds().is_empty()
    {
        f_seconds = seconds() as f32;
    }
    
    if !milliseconds().is_empty()
    {
        f_milliseconds = milliseconds() as f32;
    }*/
    
    let total_time: f32 = ((hours() as f32 * 3600.0) + (minutes() as f32 * 60.0) + seconds() as f32 + (milliseconds() as f32/ 100.0)).into();

    let mut time_under_hour: f32 = 3600.0 - total_time;
    if time_under_hour < 0.0 
    {
        time_under_hour = 0.0;
    }

    let result: f64 = time_under_hour as f64 + coins() as f64 * players() as f64;

    return result * 100.0;

}



