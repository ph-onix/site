use crate::app::Icon;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero nav-align" aria_labelledby="hero-heading">
            <div class="hero-copy">
                <h1 id="hero-heading">Project Docs</h1>
                <p>"Living documentation of software development — architecture decisions, cadence, and live demos."</p>
            </div>
            <div class="term">
                <div aria-hidden="true">
                    {Icon::VsBash.into_view()}
                    <hr/>
                </div>
                <ol role="log" aria-label="Terminal history">
                    <li>
                        <span>$ ./stats.sh</span>
                        <span>"5 active projects · last change 2d ago"</span>
                    </li>
                    <li>
                        <span>$ git log --oneline -3</span>
                        <div>
                            <span><span>a1b2c3</span> feat: add auth middleware</span>
                            <span><span>d4e5f6</span> fix: query timeout on large sets</span>
                            <span><span>g7h8i9</span> docs: update API spec</span>
                        </div>
                    </li>
                </ol>
                <div class="term-prompt">
                    <span aria-hidden="true">$</span>
                    <label for="cmd-input" class="sr-only">Terminal command</label>
                    <input id="cmd-input" type="text" spellcheck="false" autocomplete="off" autocorrect="off" autocapitalize="off" />
                </div>
            </div>
        </section>
    }
}
