use yew::prelude::*;
use web_sys::window;



#[derive(PartialEq, Properties)]
pub struct Props {
    pub addr_text: String,
    pub pub_text: String

}

pub struct Card;

impl Component for Card {

    type Message = ();
    type Properties = Props;

    fn create(_ctx:&Context<Self>)-> Self{
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{


        let onclick = _ctx.link().callback(|_| {
            let window = window().unwrap();
            let x = window.document().unwrap().get_element_by_id("view-pub").unwrap();
            x.toggle_attribute("hidden");
        });


        let onclick0 = _ctx.link().callback(|_| {
            let window = window().unwrap();
            let x = window.document().unwrap().get_element_by_id("view-pub").unwrap();
            x.toggle_attribute("hidden");
        });

        let onclick1 = _ctx.link().callback(|_| {
            let window = window().unwrap();
            let x = window.document().unwrap().get_element_by_id("view-prv").unwrap();
            x.toggle_attribute("hidden");
        });

        let onclick2 = _ctx.link().callback(|_| {
            let window = window().unwrap();
            let x = window.document().unwrap().get_element_by_id("view-prv").unwrap();
            x.toggle_attribute("hidden");
        });


        html! {
            <div class="card-holder">
                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Address"}</div>
                        <div class="card-in-de comment br animate-sh w80" align="center">{_ctx.props().addr_text.clone()}</div>
                	</div>
                </div>


                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Public Key"}</div>
                        <div class="card-in-button" align="center"><button onclick={onclick0} >{"View"}</button></div>
                        <div class="view-public" id="view-pub" hidden=true>
                            <div class="view-0" >
                                <button {onclick} >{"x"}</button>
                            </div>
                            <br/>
                            <p>{_ctx.props().pub_text.clone()}</p>
                        </div>
                	</div>
                </div>

                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Private Key"}</div>
                        <div class="card-in-button" align="center"><button onclick={onclick2}>{"View"}</button></div>
                        <div class="view-private"  id="view-prv" hidden=true>
                            <div>
                                <button onclick={onclick1} >{"x"}</button>
                            </div>
                            <br/>
                            <p>{"***********************"}</p>
                        </div>
                    </div>
                </div>
                <div class="card-type">
                	<div class="card-in">
                		<div class="card-in-d" align="center">{"Connection Status"}</div>
                        <div class="card-in-button" align="center"><button>{"STATUS"}</button></div>
                    </div>
                </div>        
            </div>
        }
    }
}
