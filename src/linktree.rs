use yew::{prelude::*, virtual_dom::AttrValue};

#[function_component(Linktree)]
pub fn linktree() -> Html {
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
        <Button prefix="fa" icon="twitch" href="//www.twitch.tv/cam_doc" text="Twitch"/>
        <Button prefix="ai" icon="orcid" href="//orcid.org/0000-0002-6694-6873" text="ORCID"/>
            </main>
    }
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
