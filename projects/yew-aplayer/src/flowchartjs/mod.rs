use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};
use flowchartjs_wasmbind::build_aplayer;

#[derive(Properties, Clone, PartialEq)]
pub struct FlowChartJSProperties {
    pub code: String,
}

pub struct FlowChartJS {
    pub props: FlowChartJSProperties,
}

impl Component for FlowChartJS {
    type Message = ();
    type Properties = FlowChartJSProperties;

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
        &build_aplayer(&t, &self.props.code);
        Html::VRef(t.into())
    }
}
