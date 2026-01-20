use dioxus::prelude::*;

#[path = "./monaco2calc/monaco2calc.rs"]
mod monaco2calc;
use dioxus_logger::tracing::Level;
use monaco2calc::Monaco2CalcMain;

#[path = "./payday3stats/payday3stats.rs"]
mod payday3stats;
use payday3stats::Payday3Stats;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/monaco-2-score-calculator")]
    Monaco2CalcMain {},
    #[route("/payday-3-stats")]
    Payday3Stats {},
}

const DEFAULT_ICON: Asset = asset!("/assets/icon.ico");
//const MAIN_CSS: Asset = asset!("/assets/main.css");


fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        document::Link {rel: "icon", href: DEFAULT_ICON}
        document::Title{ "MrMountainMan Github" }
        Router::<Route> {}
    }
}

//home page
#[component]
fn Home() -> Element {
    rsx! {
        document::Link {rel: "icon", href: DEFAULT_ICON}
        document::Title{ "MrMountainMan Github" }
        
        h2 {"Home Page!"}

        Link { to: Route::Monaco2CalcMain {}, "Monaco 2 Score Calculator"}
        br {}
        Link { to: Route::Payday3Stats {}, "Payday 3 Stats"}
    }
}