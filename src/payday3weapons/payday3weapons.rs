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
    //let mut weapons_file = use_signal(|| Vec::<Weapon>::new());

    let mut weapons_file: Vec<Weapon> = Vec::<Weapon>::new();
    let mut test_json = use_signal(|| "no json yet");
    //let mut weapons_file2;

    //let weapon_file_path = Path::new("/assets/payday3weapons/weapons.json");
    //let weapons_file = File::open(weapon_file_path).expect("could not open the weapons JSON file");
    /*
    let weapons_file_from_git = move |_| async move {
        let response = reqwest::get("https://github.com/MrMountainMan/MrMountainMan.github.io/blob/main/assets/payday3weapons/weapons.json").await.unwrap().json::<Vec<Weapon>>().await.unwrap();
        //weapons_file.set(response);
        weapons_file = response;
    };*/

    /*
    let test = async move {
        let response = reqwest::get("https://github.com/MrMountainMan/MrMountainMan.github.io/blob/main/assets/payday3weapons/weapons.json")
            .await
            .unwrap()
            .text()
            .await;
        info!("response = {response:?}");
        test_json.set(response.unwrap().as_str());
    };*/


    /*
    let test = async move {
        let response = reqwest::get("https://github.com/MrMountainMan/MrMountainMan.github.io/blob/main/assets/payday3weapons/weapons.json")
            .await
            .unwrap()
            .json::<Vec<Weapon>>()
            .await
            .unwrap();
        let response2 = reqwest::get("https://github.com/MrMountainMan/MrMountainMan.github.io/blob/main/assets/payday3weapons/weapons.json")
            .await
            .unwrap();
        weapons_file2 = response2;
    };*/

    /*
    let save = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };*/

    //let weapons: Vec<Weapon> = serde_json::from_reader(weapons_file).expect("unable to parse the weapons JSON");

    //let asd = weapons_file.read()[0].armor_penetration;
    //weapons_file.read().len();

    rsx!
    {
        //change the icon and the website title
        document::Link {rel: "icon", href: asset!("/assets/payday3weapons/pd3.ico")}
        document::Title{ "Payday 3 Weapons" }

        h1 {"Payday 3 Weapon Spreadsheet"}
        div
        {
            p{"this is some text"}
            "{weapons_file.len()}"
            "{test_json}"
            
        }
        
        
    }
}