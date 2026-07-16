use crate::api::repo_state::RepoState;
use crate::app::Icon;
use chrono::{DateTime, Utc};
use leptos::either::Either;
use leptos::prelude::*;
use std::sync::Arc;

#[server]
async fn list_repos() -> Result<Vec<RepoState>, ServerFnError> {
    let mut cache = expect_context::<crate::api::repo_cache::RepoCache>();
    cache
        .repos(None)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn ProjectCatalog() -> impl IntoView {
    // Each repo is planned to expose a webhook the server will use to cache the most recent state in Redis.
    // a row is any set of ProjectCards that have col_spans that sum to 12 or don't specify a col_span

    let repos = Resource::new(
        || (),
        |_| async move { list_repos().await.map(|v| Arc::new(v)) },
    );
    let repo_view = move || {
        Suspend::new(async move {
            let result = repos.await;
            match result {
                Ok(v) => v
                    .iter()
                    .map(|v| {
                        view! {
                            <li>
                                <ProjectCard
                                    name=v.name.clone()
                                    href=format!("/{}", v.name)
                                    last_change_utc=v.head_commit.as_ref().map(|v| v.timestamp.clone())
                                    description=v.description.clone()
                                />
                            </li>
                        }
                        .into_any()
                    })
                    .collect_view(),
                Err(e) => vec![view! { <p>{e.to_string()}</p> }.into_any()],
            }
        })
    };

    view! {
        <div class="pcatalog">
            <h2>Projects</h2>
            <Suspense fallback=move || view! { <p>Loading ???</p> }>
                <ul>{repo_view}</ul>
            </Suspense>
        </div>
    }
}

/// Compact representation of a Github repo commit state.
#[component]
fn ProjectCard(
    name: String,
    href: String,
    last_change_utc: Option<DateTime<Utc>>,
    description: Option<String>,
    #[prop(optional)] icon: Option<Icon>,
    #[prop(default = 12)] col_span: u8,
    #[prop(optional)] headlines: Option<Vec<(String, String)>>,
) -> impl IntoView {
    let fmt_last_change = match last_change_utc {
        Some(v) => Either::Left({
            let day_delta = (Utc::now().date_naive() - v.date_naive()).num_days();
            let fmt_delta = match day_delta {
                0 => "today".to_string(),
                1 => "yesterday".to_string(),
                _ => format!("{day_delta}d ago"),
            };
            view! { <span>{format!("last change {fmt_delta}")}</span> }
        }),
        None => Either::Right(view! { <></> }),
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
                {icon.map(|v| v.into_view())}
                <h3>{name}</h3>
                {fmt_last_change}
            </div>
            <p>{description}</p>
            {headline_view}
        </a>
    }
}
