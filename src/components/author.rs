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
                	<a href="https://github.com/dcnetwork"><img id="gth" src="/public/github.dark.min.svg"/><a href="https://github.com/dcnetwork"/></a>
                </div>
                <div class="author-in">
                    <p>{"Created By thelinuxpoint a.k.a Prakash Choudhary"}</p>
                </div>
                <div class="author-in">
                    <a href="https://dcnetwork.github.io"><img id="wb" src="/public/web.svg"/></a>
                </div>
            </div>
        }
    }
}
