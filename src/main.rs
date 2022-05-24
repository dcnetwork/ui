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
    pub fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}


#[function_component(App)]
pub fn app() -> Html {

    // spawn_local( async {
    //     let u = hello(String::from("prakash")).await;
    //     println!("dsdassa {:?}",u.unwrap() );
    //     // let window = window().unwrap();
    //     // window.alert_with_message("Hello Prakash");
    // });
    let u = hello(String::from("prakash")).unwrap();
    println!("{:?}",u.as_string().unwrap().as_str());
    html! {
        <div>
            <h2 class={"heading"}>{"Welcome Back, USER!"}</h2>
            
            <Card/>
            
            <div class="loader-out">
                <div class="loader">
                    <div class="face">
                        <div class="circle"></div>
                    </div>
                    <div class="face">
                        <div class="circle"></div>
                    </div>
                </div>
                <p>{"Checking ..."}</p>
            </div>

            <div align="center" class={"logo-out"}>
                <img class={"logo"} src="public/dec.png"/>
            </div>
            <Author/>
        </div>
    }
}

