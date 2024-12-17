use dioxus::prelude::*;

pub fn ClearGen() -> Element {
    rsx!
    {
        link { rel: "stylesheet", href: "/cgstylesheet.css" }
        div {
            h1 { class: "test", "GTFO Completion Screen Generator" }
        }
        div {
            label { r#for: "level_name", "Level: " }
            input {
                id: "level_name",
                r#type: "text",
                maxlength: "100",
                size: "20",
                value: "R4C2: \"Pabulum\"",
            }
            br {}
            "Time to complete: "
            input {
                id: "clear_time_hours",
                r#type: "number",
                min: 0,
                max: 99,
            }
            ":"
            input {
                id: "clear_time_minutes",
                r#type: "number",
                min: 0,
                max: 59,
            }
            ":"
            input {
                id: "clear_time_seconds",
                r#type: "number",
                min: 0,
                max: 59,
            }
        }
    }
}