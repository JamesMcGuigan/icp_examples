// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate counter_yew;
extern crate yew;
use gloo;

// This is the Yew Entry Point
#[allow(dead_code)]
fn main() {
    // DOCS: https://yew.rs/docs/next/concepts/html/events#event-bubbling
    // yew::set_event_bubbling(false);

    // yew::Renderer::<counter_yew::components::Counter>::new().render();
    yew::Renderer::<counter_yew::components::counter::CounterComponent>::with_root(
        gloo::utils::document().get_element_by_id("yew").unwrap(),
    ).render();
}
