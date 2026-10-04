use dioxus::prelude::*;
use dioxus_google_fonts::google_fonts;
use dioxus_sdk_time::{self, use_interval};
use std::time::Duration;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Shess" }
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        { google_fonts!([
            ("Inter", wght = ["200..800"]),
        ]) }
    }
}

