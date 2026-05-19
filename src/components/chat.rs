use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    username: String, // <-- Ditambahkan untuk melacak current user
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let user = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set")
            .0;
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            username: username.clone(), // <-- Disimpan di state
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen bg-gray-50 font-sans">
                // Sidebar Area
                <div class="flex-none w-72 h-screen bg-white shadow-[2px_0_15px_rgba(0,0,0,0.05)] z-10 flex flex-col">
                    <div class="text-xl font-extrabold p-6 border-b border-gray-100 text-purple-700 flex items-center gap-3">
                        <span class="text-2xl">{"🌐"}</span>
                        {"Online Users"}
                    </div>
                    <div class="overflow-y-auto flex-grow p-4 space-y-3">
                        {
                            self.users.clone().iter().map(|u| {
                                html!{
                                    <div class="flex items-center bg-gray-50 hover:bg-purple-50 rounded-2xl p-3 transition-colors duration-200 cursor-pointer border border-transparent hover:border-purple-100">
                                        <div class="relative">
                                            <img class="w-12 h-12 rounded-full shadow-sm bg-white" src={u.avatar.clone()} alt="avatar"/>
                                            <span class="absolute bottom-0 right-0 w-3.5 h-3.5 bg-green-500 border-2 border-white rounded-full"></span>
                                        </div>
                                        <div class="ml-4 flex-grow">
                                            <div class="text-sm font-bold text-gray-800">{u.name.clone()}</div>
                                            <div class="text-xs text-green-500 font-medium">{"Active now"}</div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // Chat Area
                <div class="grow h-screen flex flex-col bg-[#F3F4F6]">
                    <div class="w-full h-16 bg-white shadow-sm flex items-center px-8 z-0">
                        <div class="text-lg font-bold text-gray-800 flex items-center gap-2">
                            {"💬 Global Lounge"}
                        </div>
                    </div>

                    <div class="w-full grow overflow-y-auto p-8 flex flex-col gap-4">
                        if self.messages.is_empty() {
                            <div class="flex flex-col items-center justify-center h-full text-gray-400">
                                <div class="text-6xl mb-4">{"🍃"}</div>
                                <div class="text-2xl font-bold text-gray-500">{"It's quiet here..."}</div>
                                <div class="text-sm mt-2">{"Be the first to break the ice!"}</div>
                            </div>
                        } else {
                            {
                                self.messages.iter().map(|m| {
                                    // Cari avatar user, fallback jika belum terdaftar
                                    let user_avatar = self.users.iter().find(|u| u.name == m.from)
                                        .map(|u| u.avatar.clone())
                                        .unwrap_or_else(|| format!("https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}", m.from));
                                    
                                    let is_me = m.from == self.username;

                                    if is_me {
                                        // Tampilan Bubble Sendiri (Kanan, Ungu)
                                        html! {
                                            <div class="flex items-end self-end max-w-[70%] flex-row-reverse group">
                                                <img class="w-8 h-8 rounded-full ml-3 mb-1 shadow-sm bg-white" src={user_avatar} alt="avatar"/>
                                                <div class="p-4 bg-gradient-to-br from-purple-500 to-purple-600 text-white rounded-tl-2xl rounded-tr-2xl rounded-bl-2xl shadow-md">
                                                    <div class="text-[10px] text-purple-200 text-right mb-1 font-semibold uppercase tracking-wider">{"You"}</div>
                                                    <div class="text-sm leading-relaxed">
                                                        if m.message.ends_with(".gif") {
                                                            <img class="mt-2 rounded-xl max-w-xs" src={m.message.clone()}/>
                                                        } else {
                                                            {m.message.clone()}
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        // Tampilan Bubble Orang Lain (Kiri, Putih)
                                        html! {
                                            <div class="flex items-end self-start max-w-[70%] group">
                                                <img class="w-8 h-8 rounded-full mr-3 mb-1 shadow-sm bg-white" src={user_avatar} alt="avatar"/>
                                                <div class="p-4 bg-white text-gray-800 rounded-tl-2xl rounded-tr-2xl rounded-br-2xl shadow-md border border-gray-100">
                                                    <div class="text-[10px] text-gray-400 mb-1 font-semibold uppercase tracking-wider">{m.from.clone()}</div>
                                                    <div class="text-sm leading-relaxed">
                                                        if m.message.ends_with(".gif") {
                                                            <img class="mt-2 rounded-xl max-w-xs" src={m.message.clone()}/>
                                                        } else {
                                                            {m.message.clone()}
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>

                    // Input Area
                    <div class="w-full bg-white p-5 shadow-[0_-5px_20px_rgba(0,0,0,0.02)]">
                        <div class="flex items-center max-w-5xl mx-auto bg-gray-100 hover:bg-gray-200 transition-colors rounded-full pr-2 focus-within:ring-2 focus-within:ring-purple-300 focus-within:bg-white">
                            <input ref={self.chat_input.clone()} type="text" placeholder="Type your message here..." class="block w-full py-4 pl-6 bg-transparent outline-none text-gray-700 placeholder-gray-400" name="message" required=true autocomplete="off" />
                            <button onclick={submit} class="p-2 m-1 bg-purple-600 hover:bg-purple-700 transition-transform hover:scale-105 active:scale-95 w-12 h-12 rounded-full flex justify-center items-center shadow-lg shrink-0">
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white w-6 h-6 ml-1">
                                    <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}