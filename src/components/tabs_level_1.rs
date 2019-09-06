#![allow(warnings)]

use crate::components::comming_soon::Model as CommingSoon;
use crate::components::welcome::Model as Welcome;

use failure::Error;
use serde_derive::{Deserialize, Serialize};

use yew::format::{Json, Nothing};
use yew::services::storage::{Area, StorageService};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};

use stdweb::web::alert;
use stdweb::web::event::{BlurEvent, ClickEvent, DoubleClickEvent, HashChangeEvent, KeyPressEvent};
use stdweb::web::{document, window, Element, HtmlElement};

use stdweb::unstable::TryInto;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};

const KEY: &'static str = "valknut_engine.tabs_main.self";

pub struct Model;

struct State;

pub enum Msg {
    ClickTab,
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    Active,
}

impl Component for Model {
    type Message = ();
    type Properties = ();
    
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }
    
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    
    fn view(&self) -> Html<Self> {
        
        
        html! {
                            <>

        <nav class="tab grey lighten-2">
        <div class="nav-wrapper">

        // BUTTONS
        <ul class="row">
        
            
            <div class="col s1" >
            <button class="tablinks brand-logo welcome " onclick=|e| toggle_tab(e, ".welcome")>{"Welcome"}</button>
            </div>
            
            <div class="col s1 push-s1" >
            <button class="tablinks brand-logo home active" onclick=|e| toggle_tab(e, ".home")>{"Home"}</button>
            </div>
            
            <div class="col s1 offset-s3 " >
            <button class="tablinks brand-logo comming_soon" onclick=|e| toggle_tab(e, ".comming_soon")>{"Comming Soon"}</button>
            </div>
            
        </ul>
        
        </div>
        </nav>
        
        //SNIPPETS
        <div class="tabcontent white welcome content" >
                <Welcome/>
        </div>

        <div class="tabcontent white home content" style="display: block;">
//        <HomeTabs/>
        </div>
        
        <div class="tabcontent white comming_soon content">
        <CommingSoon/>
        </div>
        
        <div id="campaign" class="tabcontent white campaign">
        // INDIVIDUAL CAMPAIGN SELECTION GOES HERE
        </div>

                            </>
                        }
    }
}


fn toggle_tab(event: ClickEvent, tab_unique_class_name: &str) {
    use crate::stdweb::web::{IElement, IParentNode, INonElementParentNode};
    
    // Get all elements with class="tabcontent" and hide them
    for tab in document().query_selector_all(".tabcontent").unwrap() {
        let tab: Element = tab.try_into().unwrap();
        tab.set_attribute("style", "display: none;").unwrap();
    }
    
    // Get all elements with class="tablinks" and remove the class "active"
    for tab in document().query_selector_all(".tablinks").unwrap() {
        let tab: Element = tab.try_into().unwrap();
        tab.class_list().remove("active");
    }
    
    let matching_tabs = document().query_selector_all(tab_unique_class_name).unwrap();
    
    match tab_unique_class_name {
        ".welcome" => {
            for elem in matching_tabs.iter() {
                let elem: Element = elem.try_into().unwrap();
                elem.class_list().add("active");
                elem.set_attribute("style", "display: block");
            }
        }
        ".home" => {
            for elem in matching_tabs.iter() {
                let elem: Element = elem.try_into().unwrap();
                elem.class_list().add("active");
                elem.set_attribute("style", "display: block");
                document().get_element_by_id("dashboard").unwrap().set_attribute("style", "display: block;");
            }
        }
        ".campaign" => {
            for elem in matching_tabs.iter() {
                let elem: Element = elem.try_into().unwrap();
                elem.class_list().add("active");
                elem.set_attribute("style", "display: block");
            }
        }
        ".comming_soon" => {
            for elem in matching_tabs.iter() {
                let elem: Element = elem.try_into().unwrap();
                elem.class_list().add("active");
                elem.set_attribute("style", "display: block");
            }
        }
        _ => alert("Catchall WHoahw!"),
    }
}