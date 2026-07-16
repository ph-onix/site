use super::terminal::Terminal;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero nav-align" aria_labelledby="hero-heading">
            <div class="hero-copy">
                <h1 id="hero-heading">Project Docs</h1>
                <p>"Living documentation of software development — architecture decisions, cadence, and live demos."</p>
            </div>
            <Terminal/>
        </div>
    }
}
