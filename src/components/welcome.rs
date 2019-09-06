
use yew::{ShouldRender, ComponentLink, Component, Html, html, Renderable};
pub struct Model;

impl Component for Model {
	type Message = ();
	type Properties = ();
	
	fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
		Model
	}
	
	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}
}

impl Renderable<Model> for Model {
	fn view(&self) -> Html<Self> {
		html! {
            <ul class="row">
                <li class="col s12"><h2>{"Welcome Back =..."}</h2></li>
                <li class="col s12"></li>
                <li class="col s12"></li>
                <div>
                </div>
            </ul>
        }
	}
}
