use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Welcome To DC Network!"}</h2>
            <p >{"Connecting To ... 0xffffffffffffffffff"}</p>
            <div align="center" class={"logo-out"}>
                <img class={"logo"} src="public/dec.png"/>
            </div>
        </div>
    }
}
