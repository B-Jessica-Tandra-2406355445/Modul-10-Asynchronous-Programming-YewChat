use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login()-> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone()
        })
    };

    html! {
        <div class="flex min-h-screen w-screen bg-gray-900 font-sans text-white">
            <div class="hidden md:flex md:w-1/2 bg-gradient-to-br from-purple-900 via-indigo-950 to-blue-900 justify-center items-center p-12 relative">
                <div class="max-w-md z-10">
                    <div class="text-6xl mb-6">{"🚀"}</div>
                    <h1 class="text-5xl font-extrabold tracking-tight mb-4 bg-clip-text text-transparent bg-gradient-to-r from-purple-400 to-pink-300">
                        {"Connect Instantly."}
                    </h1>
                    <p class="text-lg text-indigo-200 leading-relaxed">
                        {"YewChat menghadirkan komunikasi asynchronous berperforma tinggi langsung ke browsermu, ditenagai oleh keandalan Rust dan WebAssembly."}
                    </p>
                    <div class="mt-8 flex gap-3 text-sm text-indigo-300 font-semibold bg-indigo-950 bg-opacity-60 p-4 rounded-xl border border-indigo-800">
                        <span>{"⚡ Powered by Yew Framework & WebSockets"}</span>
                    </div>
                </div>
            </div>

            <div class="w-full md:w-1/2 flex flex-col justify-center items-center p-8 bg-gray-950">
                <div class="w-full max-w-md p-8 bg-gray-900 rounded-3xl border border-gray-800 shadow-2xl">
                    <div class="md:hidden text-4xl mb-4">{"👋"}</div>
                    <h2 class="text-3xl font-bold text-white mb-2">{"Welcome Back"}</h2>
                    <p class="text-gray-400 text-sm mb-8">{"Silakan masukkan username kamu untuk bergabung ke dalam ruang obrolan global."}</p>
                    
                    <div class="flex flex-col gap-6">
                        <div class="flex flex-col gap-2">
                            <label class="text-xs font-semibold text-gray-400 uppercase tracking-wider">{"Username"}</label>
                            <input oninput={oninput} class="w-full rounded-xl p-4 bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all text-sm font-medium" placeholder="Contoh: jessica_tandra" autocomplete="off" />
                        </div>
                        
                        <Link<Route> to={Route::Chat}>
                            <button onclick={onclick} disabled={username.len()<1} class="w-full rounded-xl bg-purple-600 hover:bg-purple-700 text-white font-bold p-4 uppercase transition-all disabled:opacity-40 disabled:cursor-not-allowed shadow-lg hover:shadow-purple-900/30 flex justify-center items-center gap-2 tracking-wide text-sm">
                                    {"Join Chatroom 🚀"}
                            </button>
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}