# 002 — Landing Page Layout

The concrete UI realization of the landing page whose *what* and *why* are fixed
in [001 — Project Purpose](../001-project-purpose/design.md). This document
enumerates the **entities** of the landing page UI. Each entity worth keeping is
a `## Priority n` section, ordered by **load-bearingness to the page's job** —
conversion through ergonomics, and building-in-public — so Priority 1 is the
entity the whole page is built around and lower priorities are progressively
more peripheral or decorative.

**Scope.** This is the *landing page* only. Entities that live solely on a
project *inner* page — local/in-page nav, breadcrumb, the Diátaxis areas
themselves (Explanation / how-to / reference), hand-authored decision rationale,
embedded code snippets — are out of scope here and are not enumerated. Where a
landing entity *links into* inner-page content, that link is noted but the
inner-page surface is not specified.

**The progression dual-form principle (cross-cutting).** A progression entity is
defined by *both* a **glance form** and a **depth form**. Context selects the
*form*, not the *presence*: every progression signal appears both on the landing
card (glance form) and on the project page (depth form). Consequently we do not
pre-cut which signals the card carries — all glance forms stay available, and the
mock-and-critique loop decides empirically which forms, combinations, and layouts
read as uncrowded against real pixels.

---

## Priority 1 — Project card

**Job**: The load-bearing entity. It carries the entire conversion thesis at a
glance — relevance, outcome, and reliability — and is the composition root for
the progression entities. Get this right and the rest of the page follows; get it
wrong and no amount of surrounding polish recovers it.

**Source**: FR-009, FR-019, FR-021.

**Form (known)**: A specialization of the large card (Priority 10) — bordered,
padded, rounded panel, icon + title on the top row, description below — *plus* a
**progression summary** composing the glance forms of Priority 2–4. The entire
card is the link target into the project home (FR-021); the progression summary
deep-links into the corresponding region.

**Constraint — nested deep-links**: the whole card is a single link to the
project home (FR-009 / FR-021), but progression sub-signals must *also* deep-link
to their own targets — the cadence recency mark and the decision-log latest entry
both point into the changelog/detail (FR-021). A real link nested inside a link
is invalid HTML, so the build cannot wrap the card in one `<a>`; it must use a
layered pattern — the card's primary link as a stretched overlay (`::after`) with
the sub-links raised above it. (Recurred throughout the progression-band mocks;
the mocks render the affordance visually but do not yet resolve the structure.)

**To resolve in mock**: which progression glance forms appear, in what
arrangement and density, without crowding; visual hierarchy between description
and progression; hover treatment carried from the large card.

## Priority 2 — Cadence indicator

**Job**: Answers "does he ship consistently?" — the reliability signal.

**Source**: FR-013, FR-014, FR-015 (form is explicitly *swappable* between
candidates without changing the underlying data).

**Glance form**: a compact mark. Candidates were *number + timeframe* ("12
commits this month"), *last-commit recency*, and a *contribution heatmap*.
**Chosen for the landing card: last-commit recency**, worded **"last change Nd
ago"** — not "last shipped", because a commit is a *change*, not necessarily a
release. The *contribution-heatmap* form was tried on the card and set aside as
too heavy for a glance; it lives only in the depth form.

**Depth form**: cadence-over-time — a fuller heatmap or sparkline (the
contribution graph belongs here, on the project page, not on the card).

**To resolve in mock**: still open — where the recency mark sits (header beside
the title vs. in the foot band) and whether it carries a pressable affordance
(see the deep-link constraint under Priority 1).

## Priority 3 — Next-up

**Job**: Answers "is this alive / what is he doing now?" — the human, building-in-
public signal. The one entity that is editorial rather than git-derived.

**Source**: FR-013, FR-018, FR-019.

**Glance form**: a single line — *what* is next (with a terse *why*).

**Depth form**: the expanded next-up including full rationale (why this is next,
not only what).

**To resolve in mock**: line length and tone on the card; how it sits relative to
the description and the cadence mark.

## Priority 4 — Decision log

**Job**: A dated, verifiable **track record of judgment** — "are his calls good,
and is he honest when they're not?" This is deliberately *not* a mirror of the
GitHub commit history: the unit is a *decision or shipped milestone*, not a
commit. It is curated (substantive commits only, noise dropped), translated (each
entry a client-facing line, not a raw git message), verifiable (each entry
deep-links to the real commit), and honest (reversals/corrections are first-class
entries, never omitted). It links *down* to the commit for proof and *across* to
the Explanation docs for rationale.

**Source**: FR-016, FR-017, FR-023, FR-025, FR-026.

**Glance form**: the **latest entry's client-facing line** — its title or an
extremely short overview — with its date, deep-linking to the changelog region.
This shows *what the most recent decision actually was*, a real glimpse of
judgment. A bare count/recency chip (e.g. "23 updates · latest 2d ago") is
explicitly rejected: it measures volume, not judgment, and the decision log
exists to evidence the latter (aligns with FR-025: curated commit bodies give a
glance-level view of recent decisions).

**Depth form**: the curated dated entry list — each with date, client-facing
line, and commit link; reversals first-class; entries link into Explanation.
(The full list renders on the project page; the landing card carries only the
latest entry.)

**To resolve in mock**: latest-entry line length/truncation at card width, and
whether the latest-decision line and the next-up line read as a clear
past↔future pair rather than two similar sentences.

## Priority 5 — Section

**Job**: The page's structural unit. The landing page is a vertical stack of
sections, each separated by a bottom border with uniform vertical padding; the
question-driven order of the page *is* the order of its sections.

**Stacking**: Sections stack **vertically** down the page. Within a section, the
content defaults to **stacking rows** — the card grid (Priority 6) flows its
cards into successive horizontal rows.

**Source**: FR-003, FR-006, FR-007.

**Form (known)**: Two layouts — **(a) full-width heading**: heading (+ optional
blurb) above its card grid; **(b) split heading-column**: on large viewports,
heading + blurb + optional "explore more" link in a left column beside the card
grid, collapsing to stacked below the large breakpoint. The Projects section is
the first content section and uses layout (a) (FR-006).

**To resolve in mock**: vertical rhythm and border treatment between sections;
which sections use which layout.

## Priority 6 — Card grid

**Job**: The shared layout substrate every section's cards sit in.

**Source**: FR-008.

**Form (known)**: A 12-column grid with uniform gaps; each card declares a column
span, stacking to full width on small viewports and expanding to its span at
larger breakpoints.

**To resolve in mock**: default spans for large vs. small cards; gap sizing;
breakpoint behavior.

## Priority 7 — Hero band

**Job**: The top band — orients the visitor and sets the minimal,
building-in-public character before the first content section.

**Source**: FR-004.

**Form (known)**: A primary visual alongside a title + subtitle, optionally with a
secondary "getting-started"-style card beside it (treated as a variant of the
large card unless the mock shows it needs its own treatment).

**To resolve in mock**: what the primary visual *is* (image / graphic / live
demo); presence and content of the secondary card; balance against the Projects
section that follows.

## Priority 8 — Top nav

**Job**: Persistent chrome; moves *between* areas of the site and signals current
location.

**Source**: FR-001, FR-002, FR-028 (active state), FR-030 (responsive collapse).

**Form (known)**: A persistent top bar with the wordmark (returns to landing,
FR-002) and exactly one item — **Projects**. Active state on the current top-nav
item. Collapses to a menu on small viewports.

**To resolve in mock**: wordmark treatment; bar height and density; collapsed
menu behavior.

## Priority 9 — Footer

**Job**: Closes the page and carries contact *without* a call-to-action —
discoverable, never pushed.

**Source**: FR-005, FR-029.

**Form (known)**: Grouped helper/external links, a divider, then a meta/social
row. Contact is surfaced here (FR-029), not as a nav item or hero button.

**To resolve in mock**: link grouping; how contact sits in the meta/social row.

## Priority 10 — Large card (generic)

**Job**: The generic bordered panel — the base the project card specializes.

**Source**: FR-009.

**Form (known)**: A bordered, padded, rounded panel with an icon + title on the
top row and a description below; the entire card is the link target, with a hover
state that emphasizes the border.

**To resolve in mock**: padding and corner radius; icon size; hover treatment
(shared down to the project card).

## Priority 11 — Small card

**Job**: The lightest content entity — compact links/items in a single row.

**Source**: FR-010.

**Form (known)**: A circular bordered icon avatar + title on a single row; the
entire card is the link target, with a hover background state.

**To resolve in mock**: avatar size; row height; hover background.

---

## Assumptions

- Project cards will be kept up to date by a github webhook that listens for
  commits with an trailor (e.g. `changelog:`) and the commit will have a structured
  body which will contain values for the Project card fields; this automates the UI
  updating process and only requires me to:

  1. link a github project to my site
  2. define what a `changelog:` commit should look like and be consistent about writing them

---

Cross-cutting treatments every entity references — gathered here rather than
scattered, and *not* numbered as entities:

- **Max-width container & responsive breakpoints** — the fixed max-width,
  consistent horizontal padding, and the breakpoints the grid and nav collapse at
  (FR-003, FR-008, FR-030).
- **Typographic scale** — title / subtitle / body / meta sizes.
- **Color & border tokens** — borders are load-bearing (card edges, section
  dividers, hover emphasis).
- **Spacing rhythm** — uniform section vertical padding, grid gaps, card padding.
- **Iconography** — one icon atom in two treatments: *circular bordered avatar*
  (small card) and *squared* (large/project card top row). Shared, not a
  standalone entity.
- **State conventions** — hover/focus/active treatments (border-emphasis on large
  cards, background-fill on small cards), and the active-location indicator in nav.
