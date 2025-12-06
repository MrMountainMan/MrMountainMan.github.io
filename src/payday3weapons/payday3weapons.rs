use dioxus::{html::{input, p, th, tr}, prelude::*};
use serde::Deserialize;
use std::{fs::File, path::Path};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DamageDistance
{
    damage: f32,
    distance: f32
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CritDistance
{
    multiplier: f32,
    distance: f32
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Weapon
{
    name: String,
    damage_distance_array: Vec<DamageDistance>,
    critical_damage_multiplier_distance_array: Vec<CritDistance>,
    armor_penetration: f32
}




pub fn Payday3WeaponsMain() -> Element
{
    let weapon_file_path = Path::new("/assets/payday3weapons/weapons.json");
    let weapons_file = File::open(weapon_file_path).expect("could not open the weapons JSON file");
    let weapons: Vec<Weapon> = serde_json::from_reader(weapons_file).expect("unable to parse the weapons JSON");

    let asd = weapons[0].armor_penetration;

    rsx!
    {
        //change the icon and the website title
        document::Link {rel: "icon", href: asset!("/assets/payday3weapons/pd3.ico")}
        document::Title{ "Payday 3 Weapons" }

        h1 {"Payday 3 Weapon Spreadsheet"}
        div
        {
            p{"this is some text"}
            "{asd}"
            
        }
        
        
    }
}