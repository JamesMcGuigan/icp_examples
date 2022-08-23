use candid::Nat;
use gloo_console::log;
use yew::prelude::*;
use crate::utils::timer::now;
use ic_cdk_macros::*;

// BUG:  ic_cdk_macros require `dfx build` which is incompatible with `trunk build`
// DOCS: https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-howto/
// #[import(canister="counter")]
// struct CandidCounter;


#[derive(Debug)]
pub struct CounterComponent {
    count: Nat,
}

pub enum Msg {
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Counter::create()");
        Self {
            count: Nat::from(0),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("Counter::view()");
        html! {
            <form class="counter">
                <p><button>{ "Increment" }</button></p>
                <p>{ "Set: " }<input type="number" name="counter"/></p>
                <p>{ "Counter: " }{ self.count.clone() }</p>
            </form>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        let time_start = now();
        if first_render {
            assert!(true);
        }
        let _time_taken = (now() - time_start) / 1000.0;
        log!(format!("Fractal::rendered() = {_time_taken:.3}s").as_str());
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Counter::update()");
        match msg {
        }
    }
}
