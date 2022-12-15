use serde::Deserialize;
use yew::{prelude::*, virtual_dom::AttrValue};

#[function_component(Linktree)]
pub fn linktree() -> Html {
    let data = fetch_data("/data.json");

    html! {
	<main>
            <h1>{ "Interesting links about Pierre Marijon" }</h1>

	    <ButtonList buttons={ (*data).clone() }/>
	    </main>

    }
}

pub fn fetch_data(url: &'static str) -> UseStateHandle<Vec<ButtonData>> {
    let buttons = use_state(|| vec![]);
    {
        let buttons = buttons.clone();
        use_effect_with_deps(move |_| {
            let buttons = buttons.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_buttons: Vec<ButtonData> = gloo_net::http::Request::get(url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                buttons.set(fetched_buttons);
            });
            || ()
        }, ());
        }

    buttons
}

#[derive(Properties, PartialEq)]
pub struct ButtonListProps {
    buttons: Vec<ButtonData>
}

#[function_component(ButtonList)]
pub fn button_list(ButtonListProps { buttons }: &ButtonListProps) -> Html {
    buttons.iter().map(|button| html! {
        <Button prefix={ button.prefix.clone() } icon={ button.icon.clone() } href={ button.href.clone() } text={ button.text.clone() } />
    }).collect()
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ButtonData {
    pub prefix: String,
    pub icon: String,
    pub href: String,
    pub text: String,
}


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub prefix: AttrValue,
    pub icon: AttrValue,
    pub href: AttrValue,
    pub text: AttrValue,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = vec![
        props.prefix.clone().to_string(),
        format!("{}-{}", props.prefix.clone(), props.icon.clone()),
    ];

    html! {
    <a class={ classes!("button") } href={ props.href.clone()}>
            <i class={ classes!(class) } aria-hidden="true"></i>
    { props.text.clone() }
    </a>
    }
}
