use yew::prelude::*;

pub struct Author;

impl Component for Author{

    type Message = ();
    type Properties = ();

    fn create(_ctx:&Context<Self>)-> Self{
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{

        html! {
            <div class="author">
                <div class="author-in">
                	<img tooltip="Contribute the CODE on GitHub" src="/public/github.dark.min.svg"/><a href="https://github.com/dcnetwork"/>
                </div>
            </div>
        }
    }
}
