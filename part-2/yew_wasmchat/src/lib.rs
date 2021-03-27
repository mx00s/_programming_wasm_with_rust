#![recursion_limit = "512"]

// extern crate strum;
// #[macro_use]
// extern crate serde_derive;
// #[macro_use]
// extern crate yew;
#[macro_use]
extern crate log;
#[macro_use]
extern crate stdweb;

pub mod services;

use serde::{Deserialize, Serialize};
use services::PubnubService;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
    pub from: String,
}

pub struct Model {
    link: ComponentLink<Self>,
    alias: String,
    pending_text: String,
    messages: Vec<Message>,
    users: HashSet<String>,
}

#[derive(Debug)]
pub enum Msg {
    SendChat,
    AddMessage(Message),
    Connect,
    EnterName(String),
    UserOffline(String),
    UserOnline(String),
    UpdatePendingText(String),
    NoOp,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: <Self as yew::Component>::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            messages: Vec::new(),
            alias: "".into(),
            users: HashSet::new(),
            pending_text: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddMessage(msg) => {
                self.messages.push(msg);
            },
            Msg::UserOnline(nick) => {
                info!("Adding user {:?}", nick);
                self.users.insert(nick);
            },
            Msg::UserOffline(nick) => {
                info!("Removing user {:?}", nick);
                self.users.remove(&nick);
            },
            Msg::SendChat => {
                info!("Called send chat!");
                todo!()
            },
            Msg::Connect => {
                todo!()
            },
            Msg::EnterName(n) => {
                self.alias = n;
            },
            Msg::UpdatePendingText(s) => {
                self.pending_text = s;
            },
            Msg::NoOp => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    
    fn view(&self) -> Html {
        html! {
            <div class="wrapper",>
                <div class="chat-text",>
                    <h1>{ "Messages" }</h1><br/>
                    <ul class="message-list",>
                    { for self.messages.iter().enumerate().map(view_message) }
                    </ul>
                </div>
                <div class="users",>
                    <h1>{ "Users" }</h1><br/>
                    <ul class="user-list",>
                    { for self.users.iter().enumerate().map(view_user) }
                    </ul>
                </div>
                <div class="connect",>
                    <input placeholder="Your Name",
                        value=&self.alias,
                        oninput=self.link.callback(|e: InputData| Msg::EnterName(e.value)),/>
                    <button onclick=self.link.callback(|_| Msg::Connect),>{ "Connect" }</button>
                </div>
                <div class="text-entry",>
                    <input placeholder="Message Text",
                        class="pending-text",
                        value=&self.pending_text,
                        oninput=self.link.callback(|e: InputData| Msg::UpdatePendingText(e.value)),
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { Msg::SendChat } else { Msg::NoOp }
                        }),/>
                </div>
            </div>
        }
    }
}

fn view_message((_idx, message): (usize, &Message)) -> Html {
    html! {
        <li>
            <label>
                <span class="sender",>{"["}{&message.from}{"]"}</span>
                <span class="chatmsg",>{&message.text}</span>
            </label>
        </li>
    }
}

fn view_user((_idx, user): (usize, &String)) -> Html {
    html! {
        <li>
            <label>{ user }</label>
        </li>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    let _pubnub = PubnubService::new("publish key", "subscribe key");
    App::<Model>::new().mount_to_body();
}
