use std::collections::HashMap;
use dioxus_logger::tracing;
use regex::Regex;
use dioxus::prelude::*;

static HOURS_VALUE: GlobalSignal<String> = Global::new(|| "".to_string());
static MINUTES_VALUE: GlobalSignal<String> = Global::new(|| "".to_string());
static SECONDS_VALUE: GlobalSignal<String> = Global::new(|| "".to_string());
static MILLISECONDS_VALUE: GlobalSignal<String> = Global::new(|| "".to_string());

static HOURS_VALUE_OLD: GlobalSignal<String> = Global::new(|| "".to_string());
static MINUTES_VALUE_OLD: GlobalSignal<String> = Global::new(|| "".to_string());
static SECONDS_VALUE_OLD: GlobalSignal<String> = Global::new(|| "".to_string());
static MILLISECONDS_VALUE_OLD: GlobalSignal<String> = Global::new(|| "".to_string());
static COINS_OLD: GlobalSignal<String> = Global::new(|| "".to_string());

static LEVEL: GlobalSignal<String> = Global::new(|| "yacht_club".to_string());
static COINS: GlobalSignal<String> = Global::new(|| "".to_string());
static PLAYER_MULT: GlobalSignal<String> = Global::new(|| "8".to_string());

static SCORE: GlobalSignal<String> = Global::new(|| "".to_string());


static MAX_COIN_AMOUNTS: GlobalSignal<HashMap<&str, u32>> = Global::new(|| HashMap::from([
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



pub fn Monaco2CalcMain() -> Element
{
    let mut players = use_signal(|| 1);

rsx!
    {
        document::Link {rel: "icon", href: asset!("./assets/monaco2.ico")}
        document::Title{"Monaco 2 Score Calculator"}

        //link { rel: "stylesheet", href: "/cgstylesheet.css" }
        div
        {
            h1 {"Monaco 2 Score Calculator"}
        }
        div
        {
            //level
            label { r#for: "level_name", "Level: " }
            select
            {
                name: "level",
                id: "level_name",
                oninput: move |event| { *LEVEL.write() = event.value(); },
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

            //number of players
            br {}
            br {}
            label { r#for: "num_players", "Number of Players: " }
            select
            {
                name: "players",
                id: "num_players",
                oninput: move |event| { players.set(event.value().parse().unwrap()); *PLAYER_MULT.write() = event.value(); },
                option { value: 8, "1"}
                option { value: 6, "2"}
                option { value: 5, "3"}
                option { value: 4, "4"}
            }

            //completion time
            br {}
            br {}
            "Time to complete: "
            input {
                id: "clear_time_hours",
                r#type: "number",
                min: 0,
                max: 99,
                value: HOURS_VALUE(),
                placeholder: 0,
                oninput: move |event| { *HOURS_VALUE.write() = event.value(); },
                onkeyup: move |event| { checkOverflowValue(event, 99, "hours") },
                onkeypress: move |event| { preventNonNumericalInput(event, 99, "hours"); },
                onpaste: move |event| { event.prevent_default(); },
            }
            ":"
            input {
                id: "clear_time_minutes",
                r#type: "number",
                min: 0,
                max: 59,
                value: MINUTES_VALUE(),
                placeholder: 0,
                oninput: move |event| { *MINUTES_VALUE.write() = event.value(); },
                onkeypress: move |event| { preventNonNumericalInput(event, 99, "minutes"); },
                onpaste: move |event| { event.prevent_default(); },
            }
            ":"
            input {
                id: "clear_time_seconds",
                r#type: "number",
                min: 0,
                max: 59,
                value: SECONDS_VALUE(),
                placeholder: 0,
                oninput: move |event| { *SECONDS_VALUE.write() = event.value(); },
                onkeypress: move |event| { preventNonNumericalInput(event, 99, "seconds"); },
                onpaste: move |event| { event.prevent_default(); },
            }
            "."
            input {
                id: "clear_time_milliseconds",
                r#type: "number",
                min: 0,
                max: 999,
                value: MILLISECONDS_VALUE(),
                placeholder: 0,
                oninput: move |event| { *MILLISECONDS_VALUE.write() = event.value(); },
                onkeypress: move |event| { preventNonNumericalInput(event, 999, "milliseconds"); },
                onpaste: move |event| { event.prevent_default(); },
            }

            br {}
            br {}
            label { r#for: "coins_collected", "Coins Collected: " }
            input {
                id: "coins_collected",
                r#type: "number",
                min: 0,
                max: MAX_COIN_AMOUNTS()[&LEVEL() as &str],
                value: COINS(),
                placeholder: 0,
                oninput: move |event| { *COINS.write() = event.value(); },
                onkeypress: move |event| { preventNonNumericalInput(event, MAX_COIN_AMOUNTS()[&LEVEL() as &str], "coins"); },
                onpaste: move |event| { event.prevent_default(); },
            }


            br {}
            br {}
            button { onclick: move |_| calculateScore(), "click this button" }

            br {}
            br {}
            text { "Score: {SCORE}" }


        }

    }

}

fn checkOverflowValue(event: Event<KeyboardData>, max_value: u32, unit: &str)
{
    tracing::info!("checking for overflow");

    let parse_result;

    match unit
    {
        "hours" => {
            parse_result = HOURS_VALUE().clone().parse::<u32>();
            match parse_result {
                Ok(num) => { if num > max_value { *HOURS_VALUE.write() = HOURS_VALUE_OLD().clone() } },
                Err(e) => { if !HOURS_VALUE().clone().is_empty() { *HOURS_VALUE.write() = HOURS_VALUE_OLD().clone() } },
            }
        },
        "minutes" => *MINUTES_VALUE.write() = MINUTES_VALUE_OLD().clone(),
        "seconds" => *SECONDS_VALUE.write() = SECONDS_VALUE_OLD().clone(),
        "milliseconds" => *MILLISECONDS_VALUE.write() = MILLISECONDS_VALUE_OLD().clone(),
        "coins" => *COINS.write() = COINS_OLD().clone(),
        _ => {},
    };

}

fn preventNonNumericalInput(event: Event<KeyboardData>, max_value: u32, unit: &str)
{
    let reg = Regex::new(r"[0-9]+").unwrap();
    if !reg.is_match(&(event.key().to_string()))
    {
        event.prevent_default();
    }
    else
    {
        //save the current value to the old value
        match unit
        {
            "hours" => if !HOURS_VALUE().clone().is_empty() && HOURS_VALUE().parse::<u32>().unwrap() <= max_value {*HOURS_VALUE_OLD.write() = HOURS_VALUE().clone()} else { *HOURS_VALUE_OLD.write() = max_value.to_string()},
            "minutes" => if MINUTES_VALUE().parse::<u32>().unwrap() <= max_value {*MINUTES_VALUE_OLD.write() = MINUTES_VALUE().clone()} else { *MINUTES_VALUE_OLD.write() = max_value.to_string()},
            "seconds" => if SECONDS_VALUE().parse::<u32>().unwrap() <= max_value {*SECONDS_VALUE_OLD.write() = SECONDS_VALUE().clone()} else { *MINUTES_VALUE_OLD.write() = max_value.to_string()},
            "milliseconds" => if MILLISECONDS_VALUE().parse::<u32>().unwrap() <= max_value {*MILLISECONDS_VALUE_OLD.write() = MILLISECONDS_VALUE().clone()} else { *MILLISECONDS_VALUE_OLD.write() = max_value.to_string()},
            "coins" => if COINS().parse::<u32>().unwrap() <= max_value {*COINS_OLD.write() = COINS().clone()} else { *COINS_OLD.write() = max_value.to_string()},
            _ => {},
        };



        /*
        if format!("{}{}", event.key().to_string(), current_num).parse::<u32>().unwrap() > max_value
        {
            event.prevent_default();
        }*/
    }
}

fn calculateScore()//, hours_time: String, minutes_time: String, seconds_time: String, milliseconds_time: String)
{
    //convert all the times to numbers
    let mut hours: f32 = 0.0;
    let mut minutes: f32 = 0.0;
    let mut seconds: f32 = 0.0;
    let mut milliseconds: f32 = 0.0;

    if !HOURS_VALUE().is_empty()
    {
        hours = HOURS_VALUE().parse::<f32>().unwrap();
    }

    if !MINUTES_VALUE().is_empty()
    {
        minutes = MINUTES_VALUE().parse::<f32>().unwrap();
    }

    if !SECONDS_VALUE().is_empty()
    {
        seconds = SECONDS_VALUE().parse::<f32>().unwrap();
    }
    
    if !MILLISECONDS_VALUE().is_empty()
    {
        milliseconds = MILLISECONDS_VALUE().parse::<f32>().unwrap();
    }
    
    let total_time: f32 = (hours * 3600f32) + (minutes * 60f32) + seconds + (milliseconds / 100f32);
    
    tracing::info!(total_time);

    let mut time_under_hour: f32 = 3600.0 - total_time;
    if time_under_hour < 0.0 
    {
        time_under_hour = 0.0;
    }

    tracing::info!(time_under_hour);

    let result: f32 = time_under_hour + COINS().parse::<f32>().unwrap() * PLAYER_MULT().parse::<f32>().unwrap();

    tracing::info!(result);

}


