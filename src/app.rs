use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::api::converse;
use crate::model::conversation::{Conversation, Message};
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());
    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            user: true,
            text: new_message.clone(),
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        //TODO send message to server
        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };
            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });
    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            })
        }
    });

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Rusty Chatbox"/>
        <ChatArea conversation/>
        <TypeArea send/>


    }
}
