use aplayer_wasmbind::{build_aplayer, APlayerAudio, APlayerOptions};
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct APlayerProperties {
    pub audios: Vec<APlayerAudio>,
    #[prop_or(0.7)]
    pub volume: f32,
}

pub struct APlayer {
    pub props: APlayerProperties,
}

impl Component for APlayer {
    type Message = ();
    type Properties = APlayerProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            true => false,
            false => {
                self.props = props;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let t = yew::utils::document().create_element("div").unwrap();
        build_aplayer(&t, &self.create_aplayer());
        // Html::VRef(t.first_child().unwrap().into())
        Html::VRef(t.into())
    }
}

impl APlayer {
    pub fn create_aplayer(&self) -> APlayerOptions {
        let mut o = APlayerOptions::default();
        o.volume = self.props.volume;
        o.set_audio_list(&self.props.audios);
        return o;
    }
}
