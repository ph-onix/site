/// Manages GitHub repository commit and documentation state in a Redis data store.
///
/// A single instance is provided to the Leptos router via [`provide_context`] at startup,
/// making it available to server functions through [`use_context`] during SSR and to
/// webhook handlers when GitHub pushes updates.
pub struct RepoCache {}
