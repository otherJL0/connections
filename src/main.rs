use dioxus::prelude::*;
use rand::seq::SliceRandom;

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
    rsx! {
        div {
            class: "grid-item",
            "{word}",
        }
    }
}

#[component]
fn Grid(words: Vec<String>) -> Element {
    rsx! {
        div {
            class: "grid-container",
            for word in words {
                WordTile { word }
            }
        }
    }
}

#[component]
fn Game() -> Element {
    let mut words = use_signal(|| {
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
            "k".to_string(),
            "l".to_string(),
            "m".to_string(),
            "n".to_string(),
            "o".to_string(),
            "p".to_string(),
        ]
    });

    let deselect = move |evt| {};
    let shuffle = move |_| words.with_mut(|w| w.shuffle(&mut rand::rng()));
    let submit = move |evt| {};
    rsx! {
        Grid { words: words() }
        div {
            id: "buttons-container",
            div {
                id: "buttons",
                button { onclick: deselect, id: "deselect", "Deselect All"}
                button { onclick: shuffle, id: "shuffle", "Shuffle"}
                button { onclick: submit, id: "submit", "Submit"}
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
