use yew::prelude::*;
use ui::components::card::*;
use ui::components::author::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;


#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {

    yew::start_app::<App>();
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap())
            }
            Err(e) => {
                
            }
        }
    });
}

#[function_component(App)]
pub fn app() -> Html {

    // spawn_local( async {
    //     let u = hello(String::from("prakash")).await;
    //     println!("dsdassa {:?}",u.unwrap() );
    //     // let window = window().unwrap();
    //     // window.alert_with_message("Hello Prakash");
    // });
    // let u = hello(String::from("prakash")).unwrap();
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "".to_string());
    
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }



    let message = (*welcome).clone();
    html! {
        <div>
            <h2 class={"heading"}>{"Welcome Back, USER!"}</h2>
           
            <Card addr_text={message}/>
            <div class="loader-out">
                <div class="loader">
                    <div class="face">
                        <div class="circle"></div>
                    </div>
                    <div class="face">
                        <div class="circle"></div>
                    </div>
                </div>
                // <p>{"LoaDinG ..."}</p>
            </div>

            <div align="center" class={"logo-out"}>
                <img class={"logo"} src="public/dec.png"/>
            </div>
            <Author/>
        </div>
    }
}

