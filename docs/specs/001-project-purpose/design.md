### Priority 1 — Building in public

A publishing platform I own and control end to end, whose job is to boost my
motivation to ship things that benefit others — with full control over how the
work is presented (e.g. in-browser usage demos).

**Why this priority**: Publicity raises the stakes of finishing, which pulls
projects across the line that would otherwise stall; and owning the surface lets
me present work in richer formats than a README allows. Every other benefit of
this project (hiring signal, learning, practice) is either downstream of, or
emergent from, actually shipping public work — so the platform that enables
shipping is the root goal.

### Priority 2 — Converting freelance clients

A presentation surface that earns a prospective client's trust and lets them
reach me on their own initiative — without ever soliciting. The design converts
by *ergonomics*, not by a pitch: the layout is ordered to answer the silent questions 
a client arrives with — "can he build what I need?", "does he maintain production software?",
"was the quality of work consistent", "how reliable is the system?", "how does he work?" — 
and each question is answered at two depths: a glance-level summary and the full answer
one click into the project. The work itself does the convincing — live demos,
visible shipping cadence, usage framed project pages.

**Why this priority**: A first client converts on proof and low friction, not on
marketing, and a hard call-to-action would clash with the minimal,
building-in-public character of the site — so the conversion strategy is to
*show, not sell*. This priority shares most of its surfaces with Priority 1 (live
demos, progression tracking, project pages); it largely reframes them toward a client's questions. 
It is a way to mearsure if my choices when building in public — because the relationship runs both
ways: shipping the write public work can produce the proof that converts a client, and
optimizing the presentation for freelancing supplies concrete targets and a
quality bar to ship *toward* — the standard a client expects from a developer who
can handle production-grade systems. That standard pulls the public work upward,
so the two priorities compound rather than compete.

## Functional Requirements

**Navigation**

- **FR-001**: System will render a persistent top navigation bar with exactly: **Projects**.
- **FR-002**: The Home/wordmark will return to the landing page.
- **FR-027**: Within a project inner page, the system will render **local
  (in-page) navigation** listing the project's Diátaxis areas (Explanation,
  how-to, reference) and its changelog, so a visitor can jump directly to a
  section without scrolling. (The top nav moves *between* areas of the site;
  local nav moves *through* a project.)
- **FR-028**: Navigation will indicate current location — an **active state** on
  the current top-nav item/section, and a **breadcrumb** on project inner pages
  (e.g. Projects → *ProjectName* → *Explanation*).
- **FR-029**: Contact will be reachable **without a call-to-action**: surfaced in
  the footer meta/social row and on the About page, never as a top-nav item or
  hero button.
- **FR-030**: Both navigation tiers will be responsive — the top nav collapses to
  a menu on small viewports, and in-project local navigation collapses to a
  drawer/disclosure.

**Landing page**

- **FR-003**: The landing page will center its content in a fixed max-width
  container with consistent horizontal padding, rendered as a vertical stack of
  *sections* separated by a bottom border with uniform vertical padding.
- **FR-004**: The *hero* (top band) will place a primary visual alongside a
  title + subtitle, optionally with a secondary "getting-started"-style card
  beside it.
- **FR-005**: The *footer* will list grouped helper/external links, a divider,
  then a meta/social row.
- **FR-006**: The **Projects** section will be the first content section, using
  the full-width-heading layout.

**Sections & cards**

- **FR-007**: A section will use one of two layouts — **(a) full-width
  heading**: heading (+ optional blurb) above its card grid; or **(b) split
  heading-column**: on large viewports, heading + blurb + optional "explore
  more" link in a left column beside the card grid, collapsing to stacked below
  the large breakpoint.
- **FR-008**: Card grids will be a 12-column grid with uniform gaps; each card
  declares a column span, stacking to full width on small viewports and
  expanding to its span at larger breakpoints.
- **FR-009**: A **large card** will be a bordered, padded, rounded panel with an
  icon + title on the top row and a description below; the entire card is the
  link target, with a hover state that emphasies the border.
- **FR-010**: A **small card** will be a circular bordered icon avatar + title
  on a single row; the entire card is the link target, with a hover background
  state.

**Projects & inner pages**

- **FR-011**: Each project will have an inner page — its documentation home —
  organized along the **Diátaxis** framework: an *Explanation* area for
  system-design rationale and decisions, and *how-to / reference* areas for
  usage. The same structure is repeated across projects.
- **FR-012**: Each project's inner page will attach to its **source repository**
  and its **live deployed instance**.

**Progression tracking**

- **FR-013**: Each project will have progression data comprising a **commit
  cadence**, a **changelog** (dated entries), and a **"next up"** current-focus
  description.
- **FR-014**: System will derive both commit cadence and the changelog from the
  project's GitHub repository via the GitHub API, fetched server-side and cached
  (no per-request calls; degrades to stale-but-present if the API is
  unavailable). Git is the single source for both signals — no manual upkeep.
- **FR-015**: The card's cadence indicator will be implemented so its visual
  form is swappable between the shortlisted candidates (number + timeframe,
  last-commit recency, contribution heatmap) without changing the underlying
  data.
- **FR-016**: The changelog will be **derived from repository commits** rather
  than hand-authored: entries are selected by a curation rule that separates
  substantial commits from noise, each rendering a date, a client-facing line,
  and a link to the underlying commit. *(The curation rule — e.g. a commit
  trailer, Conventional Commits prefixes, or releases — is an open design
  decision to be settled before implementation; this FR does not commit to one.)*
- **FR-017**: System will display a changelog recency/count summary (e.g.
  "23 updates · latest 2 days ago"), computed from the derived entry set.
- **FR-018**: Each project will have a short **"next up / in development"**
  description that includes its rationale (why this is next), not only what is
  next.
- **FR-019**: Each project card will display a progression summary: the next-up
  line, a cadence indicator, and optionally changelog recency.
- **FR-020**: A project home page will display full progression detail: cadence
  over time, the full changelog, and the expanded next-up.
- **FR-021**: The card's progression summary will link into the project home
  page; the changelog recency will deep-link to the changelog region (e.g.
  `#changelog`).

**Reasoning & evidence**

- **FR-022**: Code snippets will be embeddable inside inner-page documentation,
  write-ups, and changelog entries — syntax-highlighted and copyable.
- **FR-023**: Every code snippet will be anchored / deep-linked to its real
  source in the repository at a pinned ref, so it can be verified in context and
  cannot drift out of sync (enforcing the load-bearing-code principle).
- **FR-024**: The Explanation area will support **decision records** — a
  structured, repeatable form: context → options considered → decision →
  consequences.
- **FR-025**: Curated commit bodies will provide a glance-level view of recent
  decisions; deeper rationale will compiled into the Explanation decision records.
- **FR-026**: Reversals/corrections will appear as first-class entries in the
  changelog and reasoning surfaces, not omitted.

## Assumptions

- Publishing work publicly will measurably increase my follow-through compared
  to keeping it private.
- Each project is a public GitHub repository reachable via the GitHub API.
- Substantial commits carry message bodies (and a curation marker), serving as
  the shared source for the changelog and the glance-level reasoning view.

## Core Principles

- **Show, don't sell**: the site converts through ergonomics, not solicitation.
  There is no call-to-action; good navigation and information scent let visitors
  find the proof they want and reach me on their own initiative. Contact is
  discoverable (About, footer), never pushed.
- **Question driven layout**: the landing page's sections are purpose built to to answer,
  the silent questions a visitor arrives with — relevance, then outcome, then reliability, then trust.
- **Progressive disclosure**: each question should be able to be answered at two depths — a
  glance-level (no click required) and the full answer one
  click into the project home.
- **Load-bearing code only**: every code snippet must earn its place by doing
  work the surrounding prose can't — proving a claim, demonstrating usage, or
  evidencing a decision. If a snippet can be removed without the reader losing
  something, it doesn't belong. Snippets are anchored to real source (never
  hand-pasted decoration), shown as the interesting core rather than full files,
  and never collected into a standalone gallery.
