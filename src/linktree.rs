/* std use */

/* crate use */
use yew::prelude::*;
use yew_router::prelude::*;

/* project use */
use crate::buttons::*;

#[function_component(Linktree)]
pub fn linktree() -> Html {
    let buttons_data = use_state(|| vec![]);

    {
	let buttons_data = buttons_data.clone();

	use_effect_with_deps(move |_| {
            let buttons_data = buttons_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
		let fetched_buttons_data: Vec<ButtonData> = gloo_net::http::Request::get("data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
		buttons_data.set(fetched_buttons_data);
            });
            || ()
	}, ());
    }


    html! {
	<main>
            <h1>{ "Interesting links about Pierre Marijon" }</h1>

	    <ButtonList buttons={ (*buttons_data).clone() }/>
	    </main>
    }
}
