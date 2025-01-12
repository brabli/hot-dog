use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Clone)]
struct TitleState(String);

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDoggy!".to_string()));
    rsx! {
        document::Stylesheet { href: TAILWIND }
        document::Link { rel: "icon", href: FAVICON }
        Title {}
        DogView {}
    }
}

// h1 { "HotDog! 🌭" }

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title", class: "text-2xl",
            h1 { "{title.0}" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let btn_classes = "p-2 border border-black rounded-sm bg-zinc-100 w-full";
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { class: "max-w-sm", src: img_src.cloned().unwrap_or_default() }
        }

        div { id: "buttons", class: "flex justify-between max-w-md p-4 gap-4",
            button {
                onclick: move |_| img_src.restart(),
                id: "skip",
                class: btn_classes,
                "Skip"
            }
            button { id: "save", class: btn_classes, "Save" }
        }
    }
}
