use crate::api::repo_state::Commit;
use crate::app::Icon;
use leptos::prelude::*;

#[server]
async fn stats() -> Result<(usize, String, Vec<Commit>), ServerFnError> {
    use std::sync::{Arc, RwLock};
    let ssr_state = expect_context::<Arc<RwLock<crate::api::SSRState>>>();
    let ssr_state = ssr_state
        .read()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let last_change = ssr_state
        .commits
        .get(0)
        .map_or_default(|v| v.timestamp.format("%b %d").to_string());
    let commits = ssr_state.commits.iter().take(3).cloned().collect();
    Ok((ssr_state.repos.len(), last_change, commits))
}

#[component]
pub fn Terminal() -> impl IntoView {
    let stats = Resource::new(|| (), |_| async move { stats().await });
    let stats_view = move || {
        Suspend::new(async move {
            match stats.await {
                Ok((repo_count, last_change_str, commits)) => {
                    let commit_view = commits
                        .into_iter()
                        .map(|v| {
                            view! {
                                <span><span>{v.repo_name}</span>" "{v.message}</span>
                            }
                        })
                        .collect_view();
                    view! {
                        <ol role="log" aria_label="Terminal history">
                            <li>
                                <span><span>$ ./stats.sh</span><span class="caret" aria_hidden="true"></span></span>
                                <div>
                                    <span>{repo_count}" active projects | last change "{last_change_str}</span>
                                </div>
                            </li>
                            <li>
                                <span><span>$ git log --oneline -3</span><span class="caret" aria_hidden="true"></span></span>
                                <div>
                                    {commit_view}
                                </div>
                            </li>
                        </ol>
                    }.into_any()
                }
                Err(_) => view! { <span>This not supposed to do this.</span> }.into_any(),
            }
        })
    };
    view! {
        <div class="term term-render">
            <div aria_hidden="true">
                {Icon::VsBash.into_view()}
                <hr/>
            </div>
            <Suspense fallback=move || view! { <p>Loading</p> }>
                {stats_view}
            </Suspense>
            <div>
                <span aria_hidden="true"><span>"$"</span><span class="caret"></span></span>
                <label for="cmd-input" class="sr-only">Terminal command</label>
                <input id="cmd-input" type="text" spellcheck="false" autocomplete="off" autocorrect="off" autocapitalize="off" disabled/>
            </div>
        </div>
    }
}
