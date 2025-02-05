use dioxus::{logger::tracing, prelude::*};

static CSS: Asset = asset!("/assets/main.css");
#[derive(Props, PartialEq, Clone)]
struct SquareProps {
    word: String,
    highlighted: bool,
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
        div {

        "Connection Game for Bubs"
    }
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
    }
}
