use yew::prelude::*;

pub struct Card;

impl Component for Card {

    type Message = ();
    type Properties = ();

    fn create(_ctx:&Context<Self>)-> Self{
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{

        html! {
            <div class="card-holder">
                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Address"}</div>
                	</div>
                </div>
                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Public Key"}</div>
                	</div>
                </div>
                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Private Key"}</div>
                	</div>
                </div>
                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Connection Status"}</div>
                	</div>

                </div>
            </div>
        }
    }
}
