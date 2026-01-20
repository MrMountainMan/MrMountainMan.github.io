use std::vec;
use dioxus::prelude::*;

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
}

#[derive(serde::Deserialize, Clone)]
struct CritProcessed
{
    crit: String,
    crit_distance: String,
}

#[derive(serde::Deserialize, Clone)]
struct Weapon
{
    name: String,
    damage_distance_array: Vec<DamageProcessed>,
    crit_distance_array: Vec<CritProcessed>,
    armor_pen: f32,
    pen: f32,
}


pub fn Payday3Stats() -> Element
{
    //assault rifles
    static ADELIG_JSON: Asset = asset!("/assets/payday3stats/assault_rifles/adelig_rg5.json");
    static CAR4_JSON: Asset = asset!("/assets/payday3stats/assault_rifles/car_4.json");
    static CHANIT_JSON: Asset = asset!("/assets/payday3stats/assault_rifles/chanit_s3.json");
    static KU_JSON: Asset = asset!("/assets/payday3stats/assault_rifles/ku_59.json");
    static NORTHWEST_JSON: Asset = asset!("/assets/payday3stats/assault_rifles/northwest_b9.json");
    static VF_JSON: Asset = asset!("/assets/payday3stats/assault_rifles/vf_7s.json");
    //lmgs
    static BLYSPRUTA_JSON: Asset = asset!("/assets/payday3stats/lmgs/blyspruta_mx63.json");
    //marksman rifles
    static FIK22_JSON: Asset = asset!("/assets/payday3stats/marksman_rifles/fik_22_tlr.json");
    static REINFELD900_JSON: Asset = asset!("/assets/payday3stats/marksman_rifles/fik_22_tlr.json");
    static SA_JSON: Asset = asset!("/assets/payday3stats/marksman_rifles/fik_22_tlr.json");
    static SPEARFISH_JSON: Asset = asset!("/assets/payday3stats/marksman_rifles/fik_22_tlr.json");
    //overkill_weapons
    static BESSY_JSON: Asset = asset!("/assets/payday3stats/overkill_weapons/bessy.json");
    static HET_JSON: Asset = asset!("/assets/payday3stats/overkill_weapons/het5_red_fox.json");
    static ARGES_JSON: Asset = asset!("/assets/payday3stats/overkill_weapons/m135_arges.json");
    //pistols
    static GARSTINI_JSON: Asset = asset!("/assets/payday3stats/pistols/garstini_viper_50ae.json");
    static JACKKNIFE_JSON: Asset = asset!("/assets/payday3stats/pistols/jackknife_se5.json");
    static PICCHIO_JSON: Asset = asset!("/assets/payday3stats/pistols/picchio_duro_5.json");
    static SIG40_JSON: Asset = asset!("/assets/payday3stats/pistols/signature_40.json");
    static SIG403_JSON: Asset = asset!("/assets/payday3stats/pistols/signature_403.json");
    static SP_JSON: Asset = asset!("/assets/payday3stats/pistols/sp_model_11.json");
    static STRYK_JSON: Asset = asset!("/assets/payday3stats/pistols/stryk_7.json");
    static TRIBUNE_JSON: Asset = asset!("/assets/payday3stats/pistols/tribune_32.json");
    //revolvers
    static BULLKICK_JSON: Asset = asset!("/assets/payday3stats/revolvers/bullkick_500.json");
    static CASTIGO_JSON: Asset = asset!("/assets/payday3stats/revolvers/j&m_castigo_44.json");
    static BISON_JSON: Asset = asset!("/assets/payday3stats/revolvers/sforza_bison.json");
    //shotguns
    static FSA_JSON: Asset = asset!("/assets/payday3stats/shotguns/fsa_12g.json");
    static JUSTICAR_JSON: Asset = asset!("/assets/payday3stats/shotguns/justicar.json");
    static PURSUVIANT_JSON: Asset = asset!("/assets/payday3stats/shotguns/m7_pursuviant.json");
    static MOSCONI_JSON: Asset = asset!("/assets/payday3stats/shotguns/mosconi_12classic.json");
    static REINFELD880_JSON: Asset = asset!("/assets/payday3stats/shotguns/reinfeld_880.json");
    static TAS_JSON: Asset = asset!("/assets/payday3stats/shotguns/tas_12.json");
    //smgs
    static ATK_JSON: Asset = asset!("/assets/payday3stats/smgs/atk_7.json");
    static FIKPC9_JSON: Asset = asset!("/assets/payday3stats/smgs/fik_pc9.json");
    static SG_JSON: Asset = asset!("/assets/payday3stats/smgs/sg_compact7.json");
    static WAR_JSON: Asset = asset!("/assets/payday3stats/smgs/war_45.json");
    static ZIV_JSON: Asset = asset!("/assets/payday3stats/smgs/ziv_commando.json");

    let mut weapons_signal = use_signal(|| Vec::<Weapon>::new());

    use_future(move || async move {
        //MASTER LIST OF ALL WEAPONS. MUST BE UPDATED IN ORDER TO DISPLAY NEW WEAPONS
        let ordered_weapons_list: Vec<Asset> = vec![ADELIG_JSON, CAR4_JSON, CHANIT_JSON, KU_JSON, NORTHWEST_JSON, VF_JSON,
            BLYSPRUTA_JSON,
            FIK22_JSON, REINFELD900_JSON, SA_JSON, SPEARFISH_JSON,
            BESSY_JSON, HET_JSON, ARGES_JSON,
            GARSTINI_JSON, JACKKNIFE_JSON, PICCHIO_JSON, SIG40_JSON, SIG403_JSON, SP_JSON, STRYK_JSON, TRIBUNE_JSON,
            BULLKICK_JSON, CASTIGO_JSON, BISON_JSON,
            FSA_JSON, JUSTICAR_JSON, PURSUVIANT_JSON, MOSCONI_JSON, REINFELD880_JSON, TAS_JSON,
            ATK_JSON, FIKPC9_JSON, SG_JSON, WAR_JSON, ZIV_JSON
        ];

        for weapon_json in ordered_weapons_list
        {   
            let bytes = dioxus::asset_resolver::read_asset_bytes(&weapon_json).await.unwrap();
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let name_json: serde_json::Value = json[0].get("Name").expect("Failed to parse weapon name from JSON").clone();
            let damage_distance_json: serde_json::Value = json[0]["Properties"].get("DamageDistanceArray").expect("Failed to parse damage distance array from JSON").clone();
            let crit_distance_json: serde_json::Value = json[0]["Properties"].get("CriticalDamageMultiplierDistanceArray").expect("Failed to parse critical damage distance array from JSON").clone();
            let armour_pen_json: serde_json::Value = json[0]["Properties"].get("ArmorPenetration").expect("Failed to parse armour pen value from JSON").clone();
            let pen_json: serde_json::Value = json[0]["Properties"].get("DamageDistanceArray").expect("Failed to parse pen value from JSON").clone();

            //process damage distance
            let mut damage_nodes_processed: Vec<DamageProcessed> = Vec::new();
            let damage_distance_array: Vec<DamageDistanceNode> = serde_json::from_value(damage_distance_json).unwrap_or(Vec::new());
            for node in damage_distance_array
            {
                let mut dist_string = String::from("span ");
                dist_string.push_str(&(node.Distance as i32 / 100).to_string());
                damage_nodes_processed.push(DamageProcessed { damage: node.Damage.to_string(), damage_distance: dist_string });
            }

            //process crit distance
            let mut crit_nodes_processed: Vec<CritProcessed> = Vec::new();
            let crit_distance_array: Vec<CriticalDistanceNode> = serde_json::from_value(crit_distance_json).unwrap_or(Vec::new());
            for node in crit_distance_array
            {
                let mut dist_string = String::from("span ");
                dist_string.push_str(&(node.Distance as i32 / 100).to_string());
                crit_nodes_processed.push(CritProcessed { crit: node.Multiplier.to_string(), crit_distance: dist_string });
            }

            let full_name: String = serde_json::from_value(name_json).unwrap();
            let (_, short_name) = full_name.rsplit_once('_').unwrap();

            weapons_signal.push(Weapon {
                name: short_name.to_string(),
                damage_distance_array: damage_nodes_processed,
                crit_distance_array: crit_nodes_processed,
                armor_pen: serde_json::from_value(armour_pen_json).unwrap_or(0f32),
                pen: serde_json::from_value(pen_json).unwrap_or(0f32)
            });
        }
    });

    rsx!
    {
        h3 {"Payday 3 Stats"}
        div {
            display: "grid",
            grid_auto_columns: "15px",
            grid_template_columns: "30px repeat(115, 15px)",
            padding: "1px",
            background: "black",
            gap: "1px",
            //blank
            div {
                grid_column_end: "span 10",
                background: "white",
                "name here"
            }

            //distance
            for i in 0..=20 {
                div{
                    grid_column_end: "span 5",
                    background: "white",
                    {(i*5).to_string()}
                }
            }

            //weapons
            for weapon in weapons_signal.iter()
            {
                div
                {
                    grid_column_start: 1,
                    grid_column_end: "span 10",
                    background: "white",
                    "Name: "{weapon.name.clone()}
                }
                for node in weapon.damage_distance_array.clone().iter()
                {
                    div
                    {
                        grid_column_end: node.damage_distance.clone(),
                        background: "white",
                        {node.damage.clone()}
                    }
                }
                div
                {
                    grid_column_start: 1,
                    grid_column_end: "span 10",
                    background: "white",
                    "Crit Multi: "
                }
                for node in weapon.crit_distance_array.clone().iter()
                {
                    div
                    {
                        grid_column_end: node.crit_distance.clone(),
                        background: "white",
                        {node.crit.clone()}
                    }
                }


                /*
                div
                { 
                    grid_column_end: weapon.damage_distance_array[0].damage_distance.clone(),
                    background: "white",
                    "ap: "
                    {weapon.armor_pen.to_string()}
                    " pen: "
                    {weapon.pen.to_string()}
                    " dmg: "
                    {weapon.damage_distance_array[0].damage.clone()}
                    " dist: "
                    {weapon.damage_distance_array[0].damage_distance.clone()}
                }*/
            }
            

        }
    }
}