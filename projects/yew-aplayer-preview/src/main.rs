#![recursion_limit = "1024"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_aplayer::{APlayer, APlayerAudio};

pub fn header_view() -> Html {
    let title = "APlayer for Yew";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/yew-aplayer">{"Fork me!"}</a>
    </header>
    }
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let player1 = vec![APlayerAudio::new(
            "緋色月下、狂咲之絶",
            "Hanser",
            "https://api.i-meto.com/meting/api?server=netease&type=url&id=437250672&auth=7cb69154fd2f5b8ca1e90b6443a8269b9a733b85",
            "https://api.i-meto.com/meting/api?server=netease&type=pic&id=109951162846052486&auth=4921d865b48783808b85526e78c9913db4d8b332",
            Some(
                "https://api.i-meto.com/meting/api?server=netease&type=lrc&id=437250672&auth=25eb253ac2a53b33f09b7c3011b1c6fa6d768941",
            ),
        )];

        let player2 = vec![
            APlayerAudio::new(
                "前前前世",
                "RADWIMPS",
                "https://cn-south-17-aplayer-46154810.oss.dogecdn.com/yourname.mp3",
                "https://cn-south-17-aplayer-46154810.oss.dogecdn.com/yourname.jpg",
                Some("https://cn-south-17-aplayer-46154810.oss.dogecdn.com/yourname.lrc"),
            ),
            APlayerAudio::new(
                "回レ！雪月花",
                "小倉唯",
                "https://cn-east-17-aplayer-35525609.oss.dogecdn.com/snowmoonflowers.mp3",
                "https://cn-east-17-aplayer-35525609.oss.dogecdn.com/snowmoonflowers.jpg",
                Some("https://cn-east-17-aplayer-35525609.oss.dogecdn.com/snowmoonflowers.lrc"),
            ),
        ];

        html! {
            <>
                {header_view()}
                <main>
                <h2>{"Single Audio:"}</h2>
                <APlayer audios=player1/>
                <h2>{"Multiple Audios:"}</h2>
                <APlayer audios=player2/>
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
