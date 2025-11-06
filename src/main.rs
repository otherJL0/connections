use dioxus::prelude::*;
use rand::seq::SliceRandom;

static CSS: Asset = asset!("/assets/main.css");
#[derive(Props, PartialEq, Clone)]
struct SquareProps {
    word: String,
    highlighted: bool,
}

impl SquareProps {
    fn new(word: &str, highlighted: bool) -> Self {
        Self {
            word: word.to_string(),
            highlighted,
        }
    }
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
fn WordTile(word: SquareProps) -> Element {
    rsx! {
        div {
            class: "grid-item",
            "{word.word}",
        }
    }
}

#[component]
fn Grid(words: Vec<SquareProps>) -> Element {
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
            SquareProps::new("a", false),
            SquareProps::new("b", false),
            SquareProps::new("c", false),
            SquareProps::new("d", false),
            SquareProps::new("e", false),
            SquareProps::new("f", false),
            SquareProps::new("g", false),
            SquareProps::new("h", false),
            SquareProps::new("i", false),
            SquareProps::new("j", false),
            SquareProps::new("k", false),
            SquareProps::new("l", false),
            SquareProps::new("m", false),
            SquareProps::new("n", false),
            SquareProps::new("o", false),
            SquareProps::new("p", false),
        ]
    });

    let toggle_square = move |evt| {};
    let shuffle = move |_| words.with_mut(|w| w.shuffle(&mut rand::rng()));
    let submit = move |evt| {};
    rsx! {
        Grid { words: words() }
        div {
            id: "buttons-container",
            div {
                id: "buttons",
                button { onclick: toggle_square, id: "deselect", "Deselect All"}
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
