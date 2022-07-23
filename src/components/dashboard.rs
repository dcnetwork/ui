use yew_router::prelude::*;
use yew::prelude::*;

#[function_component(DashBoard)]
pub fn dashboard() -> Html {
	html!{
		<div>
			<h>{"Goli Beta MASTI nahi ..."} </h>
			<a href="/"><button>{"Back"}</button></a>
		</div>
	}
}

