use std::collections::HashMap;
use dioxus_logger::tracing;
use regex::Regex;
use dioxus::{document::document, prelude::*};

//const COIN_AMOUNTS: HashMap<&str, i32> = HashMap::from([("yacht_club", 130),("hotel", 169),("bona_fide_sanitation", 270)]);
//const COIN_AMOUNTS: HashMap::<&str, i32> = HashMap([("yacht_club", 130),("hotel", 169),("bona_fide_sanitation", 270)]);

pub fn Monaco2CalcMain() -> Element
{
    let coin_amounts: HashMap<&str, i32> = HashMap::from([
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
        ]);

    let mut hours_value = use_signal(|| "".to_string());
    let mut minutes_value = use_signal(|| "".to_string());
    let mut seconds_value = use_signal(|| "".to_string());
    let mut milliseconds_value = use_signal(|| "".to_string());

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
                option { value: 1, "1"}
                option { value: 2, "2"}
                option { value: 3, "3"}
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
                value: hours_value,
                placeholder: 0,
                oninput: move |event| { hours_value.set(event.value());},
                onkeypress: move |event| { preventNonNumericalInput(event, hours_value(), 99); },
                onpaste: move |event| { event.prevent_default(); },
            }
            ":"
            input {
                id: "clear_time_minutes",
                r#type: "number",
                min: 0,
                max: 59,
                value: minutes_value,
                placeholder: 0,
                oninput: move |event| { minutes_value.set(event.value());},
                onkeypress: move |event| { preventNonNumericalInput(event, minutes_value(), 99); },
                onpaste: move |event| { event.prevent_default(); },
            }
            ":"
            input {
                id: "clear_time_seconds",
                r#type: "number",
                min: 0,
                max: 59,
                value: seconds_value,
                placeholder: 0,
                oninput: move |event| { seconds_value.set(event.value());},
                onkeypress: move |event| { preventNonNumericalInput(event, seconds_value(), 99); },
                onpaste: move |event| { event.prevent_default(); },
            }
            ":"
            input {
                id: "clear_time_milliseconds",
                r#type: "number",
                min: 0,
                max: 999,
                value: milliseconds_value,
                placeholder: 0,
                oninput: move |event| { milliseconds_value.set(event.value());},
                onkeypress: move |event| { preventNonNumericalInput(event, milliseconds_value(), 999); },
                onpaste: move |event| { event.prevent_default(); },
            }

        }

    }

}


fn preventNonNumericalInput(event: Event<KeyboardData>, current_num: String, max_value: i32)
{
    let reg = Regex::new(r"[0-9]+").unwrap();
    if !reg.is_match(&(event.key().to_string()))
    {
        event.prevent_default();
    }
    else
    {
        if format!("{}{}", event.key().to_string(), current_num).parse::<i32>().unwrap() > max_value
        {
            event.prevent_default();
        }
    }
}