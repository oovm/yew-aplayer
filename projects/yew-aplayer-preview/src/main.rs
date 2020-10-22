#![recursion_limit = "1024"]

use yew::{
    format::Json,
    html,
    prelude::*,
    services::storage::{Area, StorageService},
    Component, ComponentLink, Html, ShouldRender,
};
use yew_flowchartjs::FlowChartJS;

pub fn header_view() -> Html {
    let title = "KaTeX for Yew";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/yew-aplayer">{"Fork me!"}</a>
    </header>
    }
}

pub enum Event {
    Input(String),
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    input: String,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was unavailable!");
        let input = {
            match storage.restore("flow chart") {
                Json(Ok(restored_model)) => restored_model,
                _ => {
                    String::from(include_str!("readme.md"))
                }
            }
        };
        Self { link, storage, input }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(s) => {
                self.input = s;
                self.storage.store("flow chart", Json(&self.input))
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {header_view()}
                <main>
                <textarea
                     placeholder="input code:"
                     value=&self.input
                     oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                 />
                <div><label>{"flowchart.js output:"}</label></div>
                <FlowChartJS code=&self.input/>
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
