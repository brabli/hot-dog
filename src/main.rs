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
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let btn_classes = "p-2 border border-black rounded-sm bg-zinc-100 w-full";
    let skip = move |_evt| {};
    let save = move |_evt| {};
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    rsx! {
        div { id: "dogview",
            img {
                class: "max-w-sm",
                src: "{img_src}",
            }
        }

        div { id: "buttons", class: "flex justify-between max-w-md p-4 gap-4",
            button { onclick: skip, id: "skip", class: btn_classes, "Skip" }
            button { onclick: save, id: "save", class: btn_classes, "Save" }
        }
    }
}
