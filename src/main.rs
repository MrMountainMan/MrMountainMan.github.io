#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[path = "./gtfoClearGenerator/cleargen.rs"]
mod cleargen;
use cleargen::ClearGen;

#[path = "./gtfoCalc/gtfocalc.rs"]
mod gtfocalc;
use gtfocalc::CalcMain;


#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/cleargen")]
    ClearGen { },
    #[route("/gtfocalc")]
    CalcMain { },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
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
    }
}
