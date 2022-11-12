use yew::{function_component, html, Callback, Html};
use yew_router::{
    prelude::{use_history, History},
    BrowserRouter, Routable, Switch,
};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/users")]
    UsersList,
    #[at("/users/create")]
    UserCreate,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::UsersList => html! { <UsersList/> },
        Route::UserCreate => html! { <CreateUser/> },
        Route::NotFound => html! { <h1>{ "Error 404: Page not found" }</h1> },
    }
}

#[function_component(Home)]
fn home() -> Html {
    let history = use_history().unwrap();
    let users_list_callback = Callback::from(move |_| history.push(Route::UsersList));

    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={users_list_callback}>{ "Users" }</button>
        </div>
    }
}

#[function_component(UsersList)]
fn users_list() -> Html {
    let history = use_history().unwrap();
    let user_create_callback = Callback::from(move |_| history.push(Route::UserCreate));

    let history = use_history().unwrap();
    let home_callback = Callback::from(move |_| history.push(Route::Home));

    html! {
        <div>
            <h1>{ "Users" }</h1>
            <button onclick={user_create_callback}>{ "Create user" }</button>
            <button onclick={home_callback}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(CreateUser)]
fn create_user() -> Html {
    let history = use_history().unwrap();
    let home_callback = Callback::from(move |_| history.push(Route::Home));

    html! {
        <div>
            <h1>{ "Create user" }</h1>
            <button onclick={home_callback}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
