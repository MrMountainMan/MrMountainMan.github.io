#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "./assets/main.css" }
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
        br {}
        br {}
        label { r#for: "test1" }
        select { name: "thing", id: "test1",
            option { disabled: true, "Assault Rifles" }
            option { value: "car4", selected: true, "CAR-4" }
            option { value: "nwest", "Northwest B-9" }
            option { value: "ku59", "KU-59" }
            option { value: "vf7s", "VF-7S" }
            option { value: "adeilg", "Adeilg RG5" }
            option { value: "chanit", "Chanit S3" }
            option { disabled: true, "LMGs" }
            option { value: "blys", "Blyspruta MX63" }
            option { disabled: true, "Marksman Rifles" }
            option { value: "sa", "SA A144" }
            option { value: "r900s", "Reinfeld 900S" }
            option { value: "fik22", "FIK 22 TLR" }
            option { disabled: true, "SMGs" }
            option { value: "fikpc", "FIK PC9" }
            option { value: "sg7", "SG Compact-7" }
            option { value: "ziv", "Ziv Commando" }
            option { value: "war45", "WAR-45" }
            option { value: "atk7", "ATK-7" }
            option { disabled: true, "Shotguns" }
            option { value: "r880", "Reinfeld 880" }
            option { value: "mosconi", "Mosconi 12 Classic" }
            option { value: "fsa12", "FSA-12G" }
            option { value: "pursuivant", "M7 Pursuivant" }
            option { disabled: true, "Pistols" }
            option { value: "sig40", "Signature 40" }
            option { value: "stryk7", "Stryk 7" }
            option { value: "model11", "SP Model 11" }
            option { value: "sig403", "Signature 403" }
            option { value: "piccho", "Piccho Duro 5" }
            option { value: "tribune", "Tribune 32" }
            option { disabled: true, "Revolvers" }
            option { value: "castigo", "J&M Castigo 44" }
            option { value: "bison", "Sforza Bison" }
            option { value: "bullkick", "Bullkick 500" }
        }
    }
}
