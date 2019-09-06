
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
                <li class="col s12"><h2>{"Comming Soon"}</h2></li>
                <li class="col s12"></li>
                <li class="col s12"></li>
                <div>
                <p>{"WASM: wasm will be good for caching and loading data super fast, also the data can be saved long term this way on your local machine. "}
                </p>
                <p>{"Also a super cool real-time visual of incomming traffic can be displayed for your viewing pleasure and future campaign interaction so you'll be motivated to do better. WASM is chosen mainly because it's as fast as it gets for the client and also it runs in most browsers from any device."}</p>
                </div>
            </ul>
        }
	}
}
