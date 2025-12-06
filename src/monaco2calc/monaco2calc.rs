use std::{collections::HashMap, ptr::null};
use regex::Regex;
use dioxus::{html::input, prelude::*};

//use dioxus_logger::tracing;


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
        ("palace", 337),
        ("safehouse", 310),
        ("catacombs", 525),
        ("casino", 464),
        ("bonhomme", u16::MAX), 
        ("petit_bank_optional", 315), //unconfirmed
        ("museum_optional", 271), //unconfirmed
        ("office_optional", 268), //unconfirmed
        ("art_gallery_optional", 274), //unconfirmed
    ]));

    let mut level: Signal<String> = use_signal(|| "yacht_club".to_string());
    let mut player_multiplier: Signal<f32> = use_signal(|| 2f32);
    let mut coins: Signal<u16> = use_signal(|| 0);

    let mut hours: Signal<u16> = use_signal(|| 0);
    let mut minutes: Signal<u16> = use_signal(|| 0);
    let mut seconds: Signal<u16> = use_signal(|| 0);
    let mut milliseconds: Signal<u16> = use_signal(|| 0);

    rsx!
    {
        //change the icon and the website title
        document::Link {rel: "icon", href: asset!("/assets/monaco2calc/monaco2.ico")}
        document::Title{"Monaco 2 Score Calculator"}
        div
        {   
            style: "width: 600px; float: left;",
            //display header
            h1 {"Monaco 2 Score Calculator"}
            br {}
            {LevelSelector(level)}
            br {}
            br {}
            {PlayerSelector(player_multiplier)}
            br {}
            br {}
            {CoinsCollected(coins, level, coins_per_level)}"/{coins_per_level()[&level() as &str]}"
            br {}
            br {}
            {CompletionTime(hours, minutes, seconds, milliseconds)}
            br {}
            br {}
            "Final score: {CalculateScore(level, player_multiplier, coins, hours, minutes, seconds, milliseconds)}"
            br {  }

        }
        div
        {
            style: "margin-left: 620px;",
            h1 { "Greedy Score Times" }
            {GreedyTimes(level, player_multiplier, coins_per_level)}
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

pub fn PlayerSelector( mut player_multiplier: Signal<f32>) -> Element
{

    rsx!
    {
        label { r#for: "num_players", "Number of Players: " }
        div
        {
            id: "num_players",
            label { r#for: "1_player", "1" }
            input { r#type: "radio", id: "1_player", name: "player_selector", value: 2f32, checked: true, onchange: move |event| { *player_multiplier.write() = event.value().parse::<f32>().unwrap(); }, }
            label { r#for: "2_player", "2" }
            input { r#type: "radio", id: "2_player", name: "player_selector", value: 1.5f32, onchange: move |event| { *player_multiplier.write() = event.value().parse::<f32>().unwrap(); }, }
            label { r#for: "2_player", "3" }
            input { r#type: "radio", id: "3_player", name: "player_selector", value: 1.25f32, onchange: move |event| { *player_multiplier.write() = event.value().parse::<f32>().unwrap(); }, }
            label { r#for: "2_player", "4" }
            input { r#type: "radio", id: "4_player", name: "player_selector", value: 1f32, onchange: move |event| { *player_multiplier.write() = event.value().parse::<f32>().unwrap(); }, }
        }
    }
}

pub fn CoinsCollected(coins: Signal<u16>, level: Signal<String>, coins_per_level: Signal<HashMap<&str, u16>>) -> Element
{
    rsx!
    {
        label { r#for: "coins_collected", "Coins Collected: " }
        input {
            id: "coins_collected",
            r#type: "number",
            min: 0,
            //max: coins_per_level()[&level() as &str],
            value: coins,
            //oninput: move |event| { CheckValidInput(event, coins, coins_per_level()[&level() as &str]); },
            onpaste: move |event| { event.prevent_default(); },
            onkeypress: move |event| { PreventNonNumericalInput(event); },
            //onkeyup: move |_| { CheckOverflow(coins, coins_per_level()[&level() as &str]) },
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

pub fn CalculateScore(level: Signal<String>, player_multiplier: Signal<f32>, coins: Signal<u16>, hours: Signal<u16>, minutes: Signal<u16>, seconds: Signal<u16>, milliseconds: Signal<u16>) -> u128
{
    if level() == "bonhomme"
    {
        //return 0;
    }

    let total_time: f32 = ((hours() as f32 * 3600.0) + (minutes() as f32 * 60.0) + seconds() as f32 + (milliseconds() as f32/ 100.0)).into();

    let mut time_under_hour: f32 = 3600.0 - total_time;
    if time_under_hour < 0.0 
    {
        time_under_hour = 0.0;
    }

    let result: f64 = time_under_hour as f64 + coins() as f64 * (player_multiplier() * 4f32) as f64;

    //tracing::info!("time under hour: {} coins: {} player multiplier: {}", time_under_hour, coins(), player_multiplier());

    return (result * 100.0) as u128;

}

pub fn GreedyTimes(level: Signal<String>, player_multiplier: Signal<f32>, coins_per_level: Signal<HashMap<&str, u16>>) -> Element
{
    if level() == "bonhomme"
    {
        return rsx!{
            "???"
        }
    }

    let missed_0: String;
    let missed_1: String;
    let missed_2: String;
    let missed_3: String;
    let missed_4: String;
    let missed_5: String;
    let missed_6: String;

    let max_coins = 1000;//coins_per_level()[&level() as &str];

    let max_time_in_seconds: f32 = max_coins as f32 * player_multiplier();

    missed_0 = SecondsToTime(max_time_in_seconds);
    missed_1 = SecondsToTime(max_time_in_seconds - (player_multiplier * 4f32) as f32);
    missed_2 = SecondsToTime(max_time_in_seconds - (2f32 * (player_multiplier * 4f32)) as f32);
    missed_3 = SecondsToTime(max_time_in_seconds - (3f32 * (player_multiplier * 4f32)) as f32);
    missed_4 = SecondsToTime(max_time_in_seconds - (4f32 * (player_multiplier * 4f32)) as f32);
    missed_5 = SecondsToTime(max_time_in_seconds - (5f32 * (player_multiplier * 4f32)) as f32);
    missed_6 = SecondsToTime(max_time_in_seconds - (6f32 * (player_multiplier * 4f32)) as f32);

    rsx!
    {
        "Missing 0 Coins: {missed_0}"
        br {}
        "Missing 1 Coin: {missed_1}"
        br {}
        "Missing 2 Coins: {missed_2}"
        br {}
        "Missing 3 Coins: {missed_3}"
        br {}
        "Missing 4 Coins: {missed_4}"
        br {}
        "Missing 5 Coins: {missed_5}"
        br {}
        "Missing 6 Coins: {missed_6}"
    }
}

pub fn SecondsToTime(seconds: f32) -> String
{
    let remainder = seconds % 60f32;
    let remaining_seconds = seconds - remainder;
    let minutes = remaining_seconds / 60f32;

    if remainder < 10f32
    {
        return format!("{}:0{}", minutes, remainder);
    }

    return format!("{}:{}", minutes, remainder);
}



