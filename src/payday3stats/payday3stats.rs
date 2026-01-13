use std::fs;

use dioxus::prelude::*;


pub fn Payday3Stats() -> Element
{
    let car4_stats = asset!("/assets/payday3stats/car4.json");

    let car4_str = car4_stats.to_string();

    let data = fs::read_to_string(car4_str).expect("should be able to read");

    rsx!
    {
        h3 {"Payday 3 Stats"}
        p { {data} }
    }
}