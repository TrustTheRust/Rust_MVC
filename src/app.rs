use crate::components::{navbar::AppBarHeader};

//use crate::components::section::Model as Stage;

use crate::components::tabs_level_1::Model as MainTabs;
//use crate::components::home::dashboard::Model as CampaignTabs;
//use crate::components::todo::Model as ToDo;

use yew::services::storage::{Area, StorageService};
use yew::prelude::*;

pub struct Model;

#[derive(Serialize, Deserialize)]
pub struct State {
    value: String,
    edit_value: String,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
//                <ToDo/>
                <AppBarHeader/>
                <MainTabs/>
//                <AppBarBottom/>
            </>
        }
    }
}
