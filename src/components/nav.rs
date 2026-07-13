use crate::app::Icon;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

#[component]
pub fn Nav() -> impl IntoView {
    let projects = vec![("turd", "/turd")]
        .into_iter()
        .map(|(name, href)| view! { <li><NavMenuRow name href /></li> })
        .collect_view();

    let contacts = vec![
        (
            "johndoe@gmail.com",
            "mailto:johndoe@gmail.com",
            Icon::LuMail,
        ),
        ("Github", "https://github.com/ph-onix", Icon::RaGithubLogo),
    ]
    .into_iter()
    .map(|(name, href, icon)| view! { <li><NavMenuRow name href icon /></li> })
    .collect_view();

    view! {
        <div class="nav-sticky">
            <nav aria-label="Primary">
                <div>
                    <Crumbs />
                    <button popovertarget="nav-menu" aria_label="Open menu">
                        {Icon::LuMenu.into_view()}
                    </button>
                    <div id="nav-menu" class="nav-menu" popover aria_label="Menu">
                        <p>Projects</p>
                        <ul>{projects}</ul>
                        <hr/>
                        <ul>{contacts}</ul>
                    </div>
                </div>
            </nav>
        </div>
        <Outlet />
    }
}

#[component]
fn NavMenuRow(
    name: &'static str,
    href: &'static str, // follows the WHATWG URL standard to be consistent with pathname.get()
    #[prop(optional)] icon: Option<Icon>,
) -> impl IntoView {
    let location = use_location();
    let selected = move || {
        let path = location.pathname.get();
        path.starts_with(href).to_string()
    };
    view! {
        <a href=href aria_current=selected>
            {match icon { Some(v) => v.into_view(), None => view! { <></> }.into_any()}}
            <span>{name}</span>
        </a>
    }
}

#[component]
fn Crumbs() -> impl IntoView {
    let location = use_location();
    let route_view = move || match location
        .pathname
        .get()
        .trim_end_matches('/')
        .rsplit_once('/')
    {
        Some((_, "")) | None => {
            vec![view! { <li><span aria_current="page">home</span></li> }.into_any()]
        }
        Some((active_href, active_route)) => {
            let mut href = String::new();
            let mut result = vec![view! { <li><a href="/">home</a></li> }.into_any()];
            active_href
                .split("/")
                .filter(|s| !s.is_empty())
                .for_each(|route| {
                    href.push('/');
                    href.push_str(route);
                    result.push(view! { <li><a href=href.clone()>{route}</a></li> }.into_any());
                });
            result.push(
                view! { <li><span aria_current="page">{active_route}</span></li> }.into_any(),
            );
            result
        }
    };
    view! {
        <nav class="crumbs" aria-label="Breadcrumb">
            <ol>{route_view}</ol>
        </nav>
    }
}
