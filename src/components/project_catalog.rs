use crate::app::Icon;
use crate::repo_state::RepoCache;
use chrono::{NaiveDate, Utc};
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn ProjectCatalog() -> impl IntoView {
    // Each repo is planned to expose a webhook the server will use to cache the most recent state in Redis.

    // a row is any set of ProjectCards that have col_spans that sum to 12 or don't specify a col_span
    let rows = vec![
        ("Card", "/turd", "2026-07-14", "shit good ngl", Icon::VsBash),
        ("Card", "/turd", "2026-07-14", "shit good ngl", Icon::VsBash),
    ]
    .into_iter()
    .map(|(name, href, last_change_utc, body, icon)| {
        view! { <li><ProjectCard name href last_change_utc body icon/></li> }
    })
    .collect_view();

    view! {
        <div class="pcatalog">
            <h2>Projects</h2>
            <ul>{rows}</ul>
        </div>
    }
}

/// Compact representation of a Github repo commit state.
#[component]
fn ProjectCard(
    name: &'static str,
    href: &'static str,
    last_change_utc: &'static str,
    body: &'static str,
    icon: Icon,
    #[prop(default = 12)] col_span: u8,
    #[prop(optional)] headlines: Option<Vec<(String, String)>>,
) -> impl IntoView {
    let fmt_last_change = match NaiveDate::parse_from_str(last_change_utc, "%Y-%m-%d") {
        Ok(date) => Either::Left({
            let day_delta = (Utc::now().date_naive() - date).num_days();
            let fmt_delta = match day_delta {
                0 => "today".to_string(),
                1 => "yesterday".to_string(),
                _ => format!("{day_delta}d ago"),
            };
            view! { <span>{format!("last change {fmt_delta}")}</span> }
        }),
        Err(_) => Either::Right(view! { <></> }),
    };
    let headline_view = match headlines {
        Some(v) => {
            let rows = v
                .into_iter()
                .map(|(headline, content)| {
                    view! {
                        <div>
                            <dt>{headline}</dt>
                            <dd>{content}</dd>
                        </div>
                    }
                    .into_any()
                })
                .collect_view();
            Either::Left(view! { <dl>{rows}</dl> })
        }
        None => Either::Right(view! { <></> }),
    };
    view! {
        <a class=format!("col-span-{col_span}") href=href>
            <div>
                {icon.into_view()}
                <h3>{name}</h3>
                {fmt_last_change}
            </div>
            <p>{body}</p>
            {headline_view}
        </a>
    }
}
