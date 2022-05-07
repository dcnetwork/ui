use yew::prelude::*;
use ui::components::card::*;
use ui::components::author::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Welcome BACK!"}</h2>
            <Card/>
            <div align="center" class={"logo-out"}>
                <img class={"logo"} src="public/dec.png"/>
            </div>
            <Author/>
        </div>
    }
}

