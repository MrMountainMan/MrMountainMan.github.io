use std::fs::File;

fn main() {
    println!("Hello, world!");
    get_from_json("1__GEAR_Rifle_Semi.json", "Damage");
}

pub fn get_from_json(file_name: &str, key: &str)
{    
    println!("{:?}", file_name);

    let file = File::open(file_name).expect("error");
    let json: serde_json::Value  = serde_json::from_reader(file).expect("Bad JSON");
    let dmg = json.get(key).expect(format!("No key value of: {key}").as_str());

    println!("{:?}", dmg.to_string());
}