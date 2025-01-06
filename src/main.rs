use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND }
        document::Link { rel: "icon", href: FAVICON }
        Title {}
        DogView {}

    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog!" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let btn_classes = "p-2 border border-black rounded-sm bg-zinc-100 w-full";

    rsx! {
        div { id: "dogview",
            img {
                class: "max-w-sm",
                src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg",
            }
        }

        div { id: "buttons", class: "flex justify-between max-w-md p-4 gap-4",
            button { id: "skip", class: btn_classes, "Skip" }
            button { id: "save", class: btn_classes, "Save" }
        }
    }
}
