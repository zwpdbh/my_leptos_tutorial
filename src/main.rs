use leptos::*;
use leptos_router::*;

use components::demo_async::*;
use components::demo_control_flow::*;
use components::demo_error_handling::*;
use components::demo_form_and_input::*;
use components::demo_iteration::*;
use components::demo_parent_children_communication::*;
use components::demo_reactivity::*;
use components::demo_route::*;
use components::*;

mod components;
fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn DemoIndex() -> impl IntoView {
    view! {
        <h2>Demo index</h2>

        <ul>
            <li>
                <a href="/demos/:basic_component">basic components</a>
            </li>
        // <li>
        // <a href="/demos/components_and_pros">components and props</a>
        // </li>
        // <li>
        // <a href="/demos/demo_basic_iteration">basic iterator</a>
        // </li>
        // <li>
        // <a href="/demos/demo_form_and_input">form and input</a>
        // </li>
        // <li>
        // <a href="/demos/demo_error_handling">error handling</a>
        // </li>
        // <li>
        // <a href="/demos/demo_reactivity">reactivity</a>
        // </li>
        // <li>
        // <a href="/demos/demo_parent_children_communication">parent child communication</a>
        // </li>
        // <li>
        // <a href="/demos/demo_async">demo async</a>
        // </li>
        // <li>
        // <a href="/demos/control_flow">demo control flow</a>
        // </li>
        // <li>
        // <a href="/demos/route">demo route</a>
        // </li>
        </ul>
    }
}

#[component]
pub fn DemoDetail() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`.
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let component = move || match id().as_str() {
        "basic_component" => view! { <BasicComponent/> },
        _ => view! { <MessageComponent/> },
    };

    component
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>Leptos demos nav</h1>
        <Router>
            <nav>
                <ul>
                    <li>
                        <a href="/">Home</a>
                    </li>
                    <li>
                        <a href="/demos">Demo Index</a>
                    </li>
                </ul>
            </nav>
            <main>
                // all our routes will appear inside <main>
                <Routes>
                    <Route path="/" view=|| view! { <h2>"Home"</h2> }/>
                    <Route path="/demos" view=DemoIndex>
                        <Route path=":id" view=DemoDetail/>

                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <div class="select-demo">
                                        "Select a demo to see the details."
                                    </div>
                                }
                            }
                        />

                    </Route>
                    // <Route path="/demos/basic_component" view=BasicComponent/>
                    // <Route path="/demos/components_and_pros" view=ComponentsAndProps/>
                    // <Route path="/demos/demo_basic_iteration" view=DemoBasicIteration/>
                    // <Route path="/demos/demo_form_and_input" view=DemoFormAndInput/>
                    // <Route path="/demos/demo_error_handling" view=DemoErrorHandling/>
                    // <Route path="/demos/demo_reactivity" view=DemoReactivity/>
                    // <Route
                    // path="/demos/demo_parent_children_communication"
                    // view=DemoParentChildrenCommunication
                    // />
                    // <Route path="/demos/demo_async" view=DemoAsync/>
                    // <Route path="/demos/control_flow" view=DemoControlFlow/>
                    // <Route path="/demos/route" view=DemoRoute/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>

            </main>
        </Router>
    }
}
