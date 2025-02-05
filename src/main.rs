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
fn WordTile(word: String) -> Element {
    tracing::info!("Rendered with word: {}", word);
    rsx! {
        div {
            class: "grid-item",
            "{word}",
        }
    }
}

#[component]
fn Game() -> Element {
    rsx! {
        div {
            class: "grid-container",
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
            WordTile{ word: "Lily" }
            WordTile{ word: "Kippy" }
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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href : CSS }
        Title {}
        Game {}
    }
}
