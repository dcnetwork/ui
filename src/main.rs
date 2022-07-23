use yew::prelude::*;
use ui::components::card::*;
use ui::components::author::*;
use ui::components::dashboard::*;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew_router::prelude::*;


// #############################################################

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invoke_START, catch)]
    pub async fn get_addr(name: String) -> Result<JsValue, JsValue>;
}
// #############################################################

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invoke_GET_PUB, catch)]
    pub async fn get_pub(name: String) -> Result<JsValue, JsValue>;
}
// #############################################################

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    DashBoard
    
}
// #############################################################

fn update_pub_message(addr: UseStateHandle<String>, pub_key: UseStateHandle<String>,name: String) {
    spawn_local(async move {
        match get_addr("".to_string()).await {
            Ok(message) => {
                addr.set(message.as_string().unwrap())
            }
            Err(e) => {

            }
        }
    });

    spawn_local(async move {
        match get_pub("".to_string()).await {
            Ok(message) => {
                pub_key.set(message.as_string().unwrap())
            }
            Err(e) => {

            }
        }
    });

}
// #############################################################

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },

        Route::DashBoard => html! { <DashBoard/> },
        

    }
}
// #############################################################

fn main() {
    yew::start_app::<App>();
}

// #############################################################
#[function_component(Home)]
pub fn home() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "".to_string());
    let pubkey = use_state_eq(|| "".to_string());

    {
        let welcome = welcome.clone();
        let pubkey = pubkey.clone();
        use_effect_with_deps(
            move |name| {
                update_pub_message(welcome, pubkey, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let address = (*welcome).clone();

    let pubk = (*pubkey).clone();
    html!{
        <div>
            <h2 class={"heading"}>{"Welcome Back, USER!"}</h2>
            <Card addr_text={address} pub_text={pubk}/>

            // <div class="loader-out">
            //     <div class="loader">
            //         <div class="face">
            //             <div class="circle"></div>
            //         </div>
            //         <div class="face">
            //             <div class="circle"></div>
            //         </div>
            //     </div>
            //     // <p>{"LoaDinG ..."}</p>
            // </div>

            <div align="center" class={"logo-out"}>
                <img class={"logo"} src="public/dec.png"/>
            </div>
            <div class={"start-dc-out"}>
                <a href="/dashboard"><button class="start-dc">{"start"}</button></a>
            </div>
            <Author/>
        </div>

    }
}
//
#[function_component(App)]
pub fn app() -> Html {

    html! {

        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        
    }
}
// ################################################################
