
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
                <li class="col s12"><h2>{"Welcome Back Advertiser..."}</h2></li>
                <li class="col s12"></li>
                <li class="col s12"></li>
                <div>
                <p>{"I intend this machine to be the fastest in every domain and also the most intelligent, most advanced, most hands-off campaign generator the world has every seen!"}
                </p>
                <p>{"You've got idea for this project, right? If you'd like to see your ideas implemented consider supporting this project"}</p>
                </div>
            </ul>
        }
	}
}
