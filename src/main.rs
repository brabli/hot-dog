use dioxus::{logger::tracing, prelude::*};

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
    use_context_provider(|| TitleState("DOGGY".to_string()));
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
        div { id: "title",
            h1 { "{title.0}" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let btn_classes = "p-2 border border-black rounded-sm bg-zinc-100 w-full";
    let mut img_src = use_signal(|| "".to_string());
    let fetch_new = move |_| async move {
        let res = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();
        img_src.set(res.message);
    };

    rsx! {
        div { id: "dogview",
            img {
                class: "max-w-sm",
                src: "{img_src}",
            }
        }

        div { id: "buttons", class: "flex justify-between max-w-md p-4 gap-4",
            button { onclick: fetch_new, id: "skip", class: btn_classes, "Skip" }
            button { id: "save", class: btn_classes, "Save" }
        }
    }
}
