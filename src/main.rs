use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            "this is some text"
        }
    }
}
