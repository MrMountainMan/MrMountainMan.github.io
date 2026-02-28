//#[cfg(feature = "blocking")]

use std::vec;
use dioxus::{prelude::*};
use serde_json::json;

#[derive(serde::Deserialize, Clone, Copy, serde::Serialize)]
struct DamageDistanceNode
{
    Damage: f32,
    Distance: f32,
}

#[derive(serde::Deserialize, Clone, Copy, serde::Serialize)]
struct CriticalDistanceNode
{
    Multiplier: f32,
    Distance: f32,
}

#[derive(serde::Deserialize, Clone)]
struct DamageProcessed
{
    damage: String,
    damage_distance: String,
    background_colour: String,
}

#[derive(serde::Deserialize, Clone)]
struct CritProcessed
{
    crit: String,
    crit_distance: String,
    background_colour: String,
}

#[derive(serde::Deserialize, Clone)]
struct Weapon
{
    is_category: bool,
    name: String,
    damage_distance_array: Vec<DamageProcessed>,
    crit_distance_array: Vec<CritProcessed>,
    armor_pen: String,
    pen: String,
}


pub fn Payday3Stats() -> Element
{
    let mut weapons_signal = use_signal(|| Vec::<Weapon>::new());

    use_future(move || async move {
        //MASTER LIST OF ALL WEAPONS. MUST BE UPDATED IN ORDER TO DISPLAY NEW WEAPONS.
        //WEAPON NAME MUST CORRESPOND TO THE JSON FILE NAME AND INCLUDE FILE PATH       
        //CATEGORIES ARE DEFINED AS "_<Category Name>"
        let weapon_paths: Vec<&str> = vec![
            "_Assault Rifles",
            "assault_rifles/adelig_rg5",
            "assault_rifles/car_4",
            "assault_rifles/chanit_s3",
            "assault_rifles/ku_59",
            "assault_rifles/northwest_b9",
            "assault_rifles/vf_7s",
            "_LMGs",
            "lmgs/blyspruta_mx63",
            "_Marksman Rifles",
            "marksman_rifles/fik_22_tlr",
            "marksman_rifles/reinfeld_900s",
            "marksman_rifles/sa_a144",
            "marksman_rifles/spearfish_1895",
            "_Pistols",
            "pistols/garstini_viper_50ae",
            "pistols/jackknife_se5",
            "pistols/picchio_duro_5",
            "pistols/signature_40",
            "pistols/signature_403",
            "pistols/sp_model_11",
            "pistols/stryk_7",
            "pistols/tribune_32",
            "_Revolvers",
            "revolvers/bullkick_500",
            "revolvers/j&m_castigo_44",
            "revolvers/sforza_bison",
            "_Shotguns",
            "shotguns/fsa_12g",
            "shotguns/justicar",
            "shotguns/m7_pursuviant",
            "shotguns/mosconi_12_classic",
            "shotguns/reinfeld_880",
            "shotguns/tas_12",
            "_SMGs",
            "smgs/atk_7",
            "smgs/fik_pc9",
            "smgs/sg_compact_7",
            "smgs/war_45",
            "smgs/ziv_commando",
            ];

        for weapon_path in weapon_paths
        {   
            //check to see if the path is a category
            if weapon_path.starts_with("_")
            {
                weapons_signal.push(Weapon {
                    is_category: true,
                    name: weapon_path.to_owned()[1..].to_owned(),
                    damage_distance_array: vec![],
                    crit_distance_array: vec![],
                    armor_pen: "0".to_string(),
                    pen: "0".to_string(),
                });
                continue;
            }

            //get the data from the weapon path
            let raw_data = reqwest::get(format!("https://raw.githubusercontent.com/MrMountainMan/MrMountainMan.github.io/refs/heads/main/assets/payday3stats/{}.json", weapon_path))//    "" + weapon_path + ".json")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            //let bytes = dioxus::asset_resolver::read_asset_bytes(&weapon_json).await.unwrap();
            //let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            
            let json: serde_json::Value = serde_json::from_str(&raw_data).unwrap();
            
            let name_json: serde_json::Value = json[0].get("Name").unwrap_or(&json!("unknown")).clone();//.expect("Failed to parse weapon name from JSON").clone();
            let damage_distance_json: serde_json::Value = json[0]["Properties"].get("DamageDistanceArray").unwrap_or(&json!([])).clone();//.expect("Failed to parse damage distance array from JSON").clone();
            let crit_distance_json: serde_json::Value = json[0]["Properties"].get("CriticalDamageMultiplierDistanceArray").unwrap_or(&json!([])).clone();//.expect("Failed to parse critical damage distance array from JSON").clone();
            let armour_pen_json: serde_json::Value = json[0]["Properties"].get("ArmorPenetration").unwrap_or(&json!("0")).clone();//("Failed to parse armour pen value from JSON").clone();
            let pen_json: serde_json::Value = json[0]["Properties"].get("MaximumPenetrationCount").unwrap_or(&json!("0")).clone();//.expect("Failed to parse pen value from JSON").clone();

            //get and shorten the name
            let full_name: String = serde_json::from_value(name_json).unwrap();
            let (_, short_name) = full_name.rsplit_once('_').unwrap();

            //process damage distance
            let mut damage_nodes_processed: Vec<DamageProcessed> = Vec::new();
            let damage_distance_array: Vec<DamageDistanceNode> = serde_json::from_value(damage_distance_json).unwrap_or(Vec::new());
            let mut total_damage_distance_covered = 0;
            let mut damage_green_amount = 200;
            let mut damage_red_amount = 0;
            for node in damage_distance_array
            {
                let mut dist_string = String::from("span ");
                dist_string.push_str(&((node.Distance/ 100.0) as i32  - total_damage_distance_covered).to_string());
                total_damage_distance_covered = (node.Distance / 100.0) as i32;
                damage_nodes_processed.push(DamageProcessed { damage: node.Damage.to_string(), damage_distance: dist_string, background_colour: format!("rgb({},{},0", damage_red_amount, damage_green_amount) });
                damage_green_amount -= 50;
                damage_red_amount += 50;
            }

            //process crit distance
            let mut crit_nodes_processed: Vec<CritProcessed> = Vec::new();
            let crit_distance_array: Vec<CriticalDistanceNode> = serde_json::from_value(crit_distance_json).unwrap_or(Vec::new());
            let mut total_crit_distance_covered = 0;
            let mut crit_green_amount = 200;
            let mut crit_red_amount = 0;
            for node in crit_distance_array
            {
                let mut dist_string = String::from("span ");
                dist_string.push_str(&((node.Distance / 100.0) as i32 - total_crit_distance_covered).to_string());
                total_crit_distance_covered = (node.Distance / 100.0) as i32;
                crit_nodes_processed.push(CritProcessed { crit: node.Multiplier.to_string(), crit_distance: dist_string, background_colour: format!("rgb({},{},0", crit_red_amount, crit_green_amount) });
                crit_green_amount -= 50;
                crit_red_amount += 50;
            }            

            weapons_signal.push(Weapon {
                is_category: false,
                name: short_name.to_string(),
                damage_distance_array: damage_nodes_processed,
                crit_distance_array: crit_nodes_processed,
                armor_pen: serde_json::from_value(armour_pen_json).unwrap_or("0".to_string()),
                pen: serde_json::from_value(pen_json).unwrap_or("0".to_string()),
            });
        }
    });

    rsx!
    {
        h3 { "Payday 3 Stats" }

        div {
            display: "grid",
            grid_auto_columns: "15px",
            grid_template_columns: "30px repeat(115, 15px)",
            padding: "1px",
            background: "black",
            gap: "1px",

            //weapons
            for weapon in weapons_signal.iter() {
                div {
                    grid_column_start: 1,
                    grid_column_end: "span 10",
                    background: "white",
                    if !weapon.is_category
                    {
                        "Name: "
                    }
                    {weapon.name.clone()}
                }
                if weapon.is_category
                {
                    for i in 0..=20 {
                        div { grid_column_end: "span 5", background: "white", {(i * 5).to_string()}}
                    }    
                } else {
                    for node in weapon.damage_distance_array.clone().iter() {
                    div {
                        grid_column_end: node.damage_distance.clone(),
                        background: node.background_colour.clone(),
                        {node.damage.clone()}
                    }
                    }
                    div {
                        grid_column_start: 1,
                        grid_column_end: "span 10",
                        background: "white",
                        "Crit Multi: "
                    }
                    for node in weapon.crit_distance_array.clone().iter() {
                        div {
                            grid_column_end: node.crit_distance.clone(),
                            background: node.background_colour.clone(),
                            {node.crit.clone()}
                        }
                    }
                } 
            }
        }
    }
}