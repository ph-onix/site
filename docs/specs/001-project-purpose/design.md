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

## Functional Requirements

**Navigation**

- **FR-001**: System will render a persistent top navigation bar with exactly:
  **Home**, **Projects**, **About**.
- **FR-002**: The Home/wordmark will return to the landing page.

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

- **FR-011**: Each project will have an inner page (its presentation & documentation home).

**Progression tracking**

- **FR-013**: Each project will have progression data comprising a **commit
  cadence**, a **changelog** (dated entries), and a **"next up"** current-focus
  description.
- **FR-014**: System will derive commit cadence from the project's git history
  (no manual upkeep).
- **FR-015**: The card's cadence indicator will be implemented so its visual
  form is swappable between the shortlisted candidates (number + timeframe,
  last-commit recency, contribution heatmap) without changing the underlying
  data.
- **FR-016**: Each project will support a changelog of dated entries (date +
  short description).
- **FR-017**: System will display a changelog recency/count summary (e.g.
  "23 updates · latest 2 days ago").
- **FR-018**: Each project will have a short **"next up / in development"**
  description.
- **FR-019**: Each project card will display a progression summary: the next-up
  line, a cadence indicator, and optionally changelog recency.
- **FR-020**: A project home page will display full progression detail: cadence
  over time, the full changelog, and the expanded next-up.
- **FR-021**: The card's progression summary will link into the project home
  page; the changelog recency will deep-link to the changelog region (e.g.
  `#changelog`).

## Assumptions

- Publishing work publicly will measurably increase my follow-through compared
  to keeping it private.
