use std::fs::File;
use serde::Deserialize;

use dioxus::prelude::*;

#[derive(Deserialize)]
struct WeaponData
{
    damage: f32,
    damagefalloff: Vec<f32>,
    staggerdamagemulti: f32,
    precisiondamagemulti: f32,
    defaultclipsize: i32,
    defaultreloadtime: f32,
    costofbullet: f32,
    shotdelay: f32,
    piercingbullets: bool, 
    piercingdamagecountlimit: i32,
    hipfirespread: f32,
    aimspread: f32
}

//const RIFLE: Asset = asset!("src/gtfoCalc/calcData/GameData_ArchetypeDataBlock/1__GEAR_Rifle_Semi.json")

pub fn CalcMain() -> Element
{

    rsx!
    {
        div {
            h1 { "GTFO Calculator" }

            {CreateSpreadSheet()}
        }
    }
}

pub fn CreateSpreadSheet() -> Element
{
    rsx!
    {
        div {
            "Damage: "
            {GetFromJson("1__GEAR_Rifle_Semi.json", "Damage")}
        }
        div {
            class: "grid",
            display: "inline-grid",
            grid_template_columns: "auto, auto, auto",
            div { grid_column: "1 / 2", {GetFromJson("1__GEAR_Rifle_Semi.json", "Damage")} }
            div { grid_column: "2 / 3", "thing 2" }
            div { grid_column: "3 / 4", "thing 3" }
            div { "thing 4" }
            div { "thing 5" }
            div { "thing 6" }
            div { "thing 7" }
            div { "thing 8" }
            div { "thing 9" }
        }
    }
}

pub fn GetFromJson(file_name: &str, key: &str) -> Element
{
    
    //let file_string = format!("C:\\Users\\James\\Documents\\MrMountainMan.github.io\\src\\gtfoCalc\\calcData\\GameData_ArchetypeDataBlock\\{file_name}");
    //let file = File::open(file_name)?;//.expect("Bad file");
    //let reader = std::io::BufReader::new(file);
    //let json2: Result<R, T> = serde_json::from_reader(reader)?;
    //let dmg2 = json2.get(key);
    
    //let rifle = asset!("src/gtfoCalc/calcData/GameData_ArchetypeDataBlock/1__GEAR_Rifle_Semi.json");
    let path = std::path::Path::new("src/gtfoCalc/calcData/GameData_ArchetypeDataBlock/1__GEAR_Rifle_Semi.json");

    //let file = File::open(format!("src/gtfoCalc/calcData/{}", file_name))?;
    let file = File::open(&path)?;
    let json: serde_json::Value  = serde_json::from_reader(file).expect("Bad JSON");
    let dmg = json.get(key).expect(format!("No key value of: {key}").as_str());

    rsx!
    {
        {dmg.to_string()}
    }
}

/*
fn read_user_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<WeaponData, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}*/