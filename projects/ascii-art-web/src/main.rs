#![recursion_limit = "1024"]
mod braille_view;
mod global;

use crate::global::{GlobalSettings, Scene};
use yew::{
    format::Json,
    html,
    services::{storage::Area, StorageService},
    Component, ComponentLink, Html, ShouldRender,
};

const KEY: &str = "ascii-art";

#[derive(Debug)]
pub enum Event {
    SwitchTo(Scene),
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: GlobalSettings,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let state = match storage.restore(KEY) {
            Json(Ok(state)) => state,
            _ => GlobalSettings::default(),
        };
        Self { link, storage, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::SwitchTo(scene) => {
                self.state.scene = scene;
                self.storage.store(KEY, Json(&self.state))
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.state.scene {
            Scene::AsciiArt => html! {
            <>
                {self.navbar_view()}
            </>
            },
            Scene::BrailleArt => html! {
            <>
                {self.navbar_view()}
                {self.braille_view()}
            </>
            },
            Scene::EmojiArt => html! {
            <>
                {self.navbar_view()}
            </>
            },
        }
    }
}

impl Model {
    pub fn navbar_view(&self) -> Html {
        html! {
        <nav class="navbar ">
            <div class="container">
                <div class="navbar-brand">
                    <a class="navbar-item">
                        <img src="https://bulma.io/images/bulma-type-white.png" alt="Logo"/>
                    </a>
                    <span class="navbar-burger burger">
                      <span></span>
                      <span></span>
                      <span></span>
                    </span>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-end">
                    {self.switch_to_ascii()}
                    {self.switch_to_braille_art()}
                    {self.switch_to_emoji()}
                    </div>
                </div>
            </div>
        </nav>
        }
    }
    fn switch_to_ascii(&self) -> Html {
        let class = match self.state.scene {
            Scene::AsciiArt => "navbar-item is-active",
            _ => "navbar-item",
        };
        html! {
            <a class=class id="title"
                onclick=self.link.callback(|_| Event::SwitchTo(Scene::AsciiArt))
            >
            {"AsciiArt"}
            </a>
        }
    }
    fn switch_to_emoji(&self) -> Html {
        let class = match self.state.scene {
            Scene::EmojiArt => "navbar-item is-active",
            _ => "navbar-item",
        };
        html! {
            <a class=class id="title"
                onclick=self.link.callback(|_| Event::SwitchTo(Scene::EmojiArt))
            >
            {"EmojiArt"}
            </a>
        }
    }
    fn switch_to_braille_art(&self) -> Html {
        let class = match self.state.scene {
            Scene::BrailleArt => "navbar-item is-active",
            _ => "navbar-item",
        };
        html! {
            <a class=class id="title"
                onclick=self.link.callback(|_| Event::SwitchTo(Scene::BrailleArt))
            >
            {"braille_view"}
            </a>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
