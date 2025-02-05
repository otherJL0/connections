use dioxus::{logger::tracing, prelude::*};

static CSS: Asset = asset!("/assets/main.css");
#[derive(Props, PartialEq, Clone)]
struct SquareProps {
    word: String,
    highlighted: bool,
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "😺 Connecticat 😺" }
        }
    }
}

#[component]
fn ConnectionsApp(word: String) -> Element {
    tracing::info!("Rendered with word: {}", word);
    rsx! {
        div {
            class: "grid-item",
            "{word}",
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href : CSS }
        Title {}
        div {
            class: "grid-container",
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
            ConnectionsApp{ word: "Lily" }
            ConnectionsApp{ word: "Kippy" }
        }
        div {
            id: "buttons-container",
            div {
                id: "buttons",
                button { id: "deselect", "Deselect All"}
                button { id: "shuffle", "Shuffle"}
                button { id: "submit", "Submit"}
            }
        }
    }
}
