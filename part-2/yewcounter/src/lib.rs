use wasm_bindgen::prelude::*;
use yew::{prelude::*, services::console::ConsoleService};

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    ConsoleService::log("Bulk action");
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        // Q16: Why does rendering the current date in a yew Component panic?

        html! {
            <div>
                <nav class="menu",>
                    <button onclick=self.link.callback(|_| Msg::Increment)>{ "Increment" }</button>
                    <button onclick=self.link.callback(|_| Msg::Decrement)>{ "Decrement" }</button>
                    <button onclick=self.link.callback(|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]))>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                // <p>{ stdweb::web::Date::new().to_string() }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
