// This line allows non-snake-case identifiers, which means you can use camelCase or PascalCase names without warnings
#![allow(non_snake_case)]

use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props)]
pub struct AppLayoutProps<'a> {
    title: &'a str,
    children: Element<'a>,
}

// using & for children and not for title in the Layout function, see below page for reason
//  Scope provides access to the props, state, and context of a component and {{ Element is HTML elements and all these things are provided by prelude::*
   
    // rsx! is like return() of React JS
pub fn Layout<'a>(cx: Scope<'a, AppLayoutProps<'a>>) -> Element {
    cx.render(rsx!(
        {
            LazyNodes::new(|f| f.text(format_args!("<!DOCTYPE html><html lang='en'>")))
        }
        head {
            title {
                "{cx.props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
        }
        body {
            &cx.props.children
        }
    ))
}



/* The reason we use & with children is because the rsx! macro expects a reference to an element as a child node, not an element itself.This is because the rsx! macro expects child nodes to be either text nodes, lazy nodes, or component nodes. An element is not one of those types, so we need to use a reference to tell the rsx! macro that we want to use the element as a child node.  This is a rule that the rsx! macro has for elements, but not for other types like string slices or components.

The reason we don’t use & with title is because the rsx! macro can handle a string slice as a text node, without needing a reference. This is another rule that the rsx! macro has for string slices, but not for other types like elements or components.

So, the use of & or not is not related to ownership or scope. It is related to the rules and limitations of the rsx! macro. The rsx! macro has different rules for different types of values that we pass to it */
