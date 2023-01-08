use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonListProps {
    pub buttons: Vec<ButtonData>,
}

#[function_component(ButtonList)]
pub fn button_list(ButtonListProps { buttons }: &ButtonListProps) -> Html {
    buttons.iter().map(|button| html! {
        <Button prefix={ button.prefix.clone() } icon={ button.icon.clone() } href={ button.href.clone() } text={ button.text.clone() } />
    }).collect()
}

#[derive(Clone, PartialEq, serde::Deserialize)]
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
