use stdweb::web::event::ClickEvent;
use stdweb::web::alert as alert;
use stdweb::web::{document, Element};
use stdweb::unstable::TryInto;

fn toggle_tab(event: ClickEvent, tab_unique_class_name: &str) {
	let i = 0;
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
		"dashboard" => {
			alert("sad123f");
			let active_tab = document().get_element_by_id("dashboard").unwrap();
			active_tab.set_attribute("style", "display: block;");
			
			for elem in matching_tabs.iter() {
				let elem: Element = elem.try_into().unwrap();
				elem.class_list().add("active");
				elem.set_attribute("style", "display: block");
			}
		}
		"campaigns" => {
			alert("dsf");
			for elem in matching_tabs.iter() {
				let elem: Element = elem.try_into().unwrap();
				elem.class_list().add("active");
				elem.set_attribute("style", "display: block");
			}
		}
		_ => alert("Catchall WHoahw!"),
	}
}