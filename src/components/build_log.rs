use crate::app::Icon;
use leptos::prelude::*;

#[component]
pub fn BuildLog() -> impl IntoView {
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
            <ol>
                <li>
                    <a href="#">
                        <span>Jun 24</span>
                        <span>Quill</span>
                        "Append-only log over CRDTs for sync"
                        {Icon::Link.into_view()}
                    </a>
                </li>
            </ol>
        </div>
    }
}
