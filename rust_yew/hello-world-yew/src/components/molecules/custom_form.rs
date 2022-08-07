use gloo::console::log;
use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;


#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_changed = Callback::from(|username| {
        log!("username changed in custom form", username);
    });
    html! {
    <form>
       <TextInput name="username"  handle_onchange={username_changed}/>
       <CustomButton label="Submit" />
    </form>
    }
}