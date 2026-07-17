use crate::{api::repo_state::Commit, app::Icon};
use chrono::{DateTime, Utc};
use leptos::prelude::*;
use std::sync::Arc;

#[server]
async fn list_commits() -> Result<Vec<Commit>, ServerFnError> {
    let mut cache = expect_context::<crate::api::repo_cache::RepoCache>();
    cache
        .commits(None, 7)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn BuildLog() -> impl IntoView {
    let commits = Resource::new(
        || (),
        |_| async move { list_commits().await.map(|v| Arc::new(v)) },
    );
    let commit_view = move || {
        Suspend::new(async move {
            match commits.await {
                Ok(v) => {
                    if v.len() == 0 {
                        return Fallback().into_any();
                    }
                    let v = v
                    .iter()
                    .map(|c| {
                        view! {
                            <Entry 
                                repo_name=c.repo_name.clone()
                                href=format!("https://api.github.com/repos/ph-onix/{}/commits/{}", c.repo_name, c.id)
                                message=c.message.clone()
                                timestamp=c.timestamp.clone()
                            />
                        }
                    })
                    .collect_view();
                    view! { <ol>{v}</ol> }.into_any()
                }
                Err(_) => view! { <span>Well damn, looks like something broke.</span> }.into_any(),
            }
        })
    };

    view! {
        <div class="bl">
            <div>
                <h2>Build log</h2>
                <p>"Dated notes on what I'm building and why."</p>
                <a href="#">
                    "See the full log"
                    {Icon::LuChevron.into_view()}
                </a>
            </div>
            <Suspense fallback=move || Fallback()>
                {commit_view}
            </Suspense>
        </div>
    }
}

#[component]
fn Entry(
    mut repo_name: String,
    href: String,
    message: String,
    timestamp: DateTime<Utc>,
) -> impl IntoView {
    repo_name.get_mut(0..1).map(|s| s.to_uppercase());
    view! {
        <li>
            <a href=href>
                <span>{timestamp.format("%b %d").to_string()}</span>
                <span>{repo_name}</span>
                {message}
                {Icon::Link.into_view()}
            </a>
        </li>
    }
}

#[component]
fn Fallback() -> impl IntoView {
    view! {
        <ol class="bl-skel">
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
            <li>
                <div>
                    <span></span>
                    <span><span></span><span></span><span></span><span></span><span></span></span>
                    <span></span>
                </div>
            </li>
        </ol>
    }
}
