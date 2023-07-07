use crate::layout::Layout;
use db::User;          // cargo add --path ../db
use dioxus::prelude::*;

struct Props {
    users: Vec<User>
}

// For lifetime parameters, see below page
// Take a Vec<User> and create an HTML table.
pub fn users(users: Vec<User>) -> String {

    // Inner function to create our rsx! component,  rsx! is like return() of React JS
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {    // <-- Use our layout
                title: "Users Table",
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Email" }
                        }
                    }
                    tbody {
                        cx.props.users.iter().map(|user| rsx!(
                            tr {
                                td {
                                    strong {
                                        "{user.id}"
                                    }
                                }
                                td {
                                    "{user.email}"
                                }
                            }
                        ))
                    }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(
        app,
        Props {
            users
        },
    );
    let _ = app.rebuild(); // see below
    dioxus::ssr::render_vdom(&app)
}


/* For more info ---> please check Dioxus_users.txt file */
