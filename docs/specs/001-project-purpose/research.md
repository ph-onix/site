# Do personal websites measurably help land a SWE internship? A cited research report

> Research conducted 2026-06-26 to inform the internship/resume motivation in [design.md](./design.md).
> Scope: backend, ML, and low-level/systems internships (Fall 2026 / Summer 2027), with generic SWE as the abundant fallback.

## Bottom-line verdict

**For hiring specifically, a personal website is a low-to-marginal ROI activity — and the best direct evidence we have says so plainly.** The single most on-point study (a survey of ~60+ hiring managers) found that while ~93% would *look* at a portfolio site if given one, ~51% said a candidate's chances are **not** lower without one, and the author's blunt conclusion was that the website is "the container, not the content" — the projects inside it do the work, not the site. [profy.dev](https://profy.dev/article/portfolio-websites-survey)

The honest framing for this situation:

- **The site itself moves the needle very little.** What moves the needle is *what you put on it* — real, shipped, demonstrable engineering — and those same artifacts (GitHub repos, a deployed service, technical write-ups) carry their signal whether or not they live behind a custom Rust/Leptos site.
- **For backend/ML/systems roles specifically, design polish is nearly irrelevant.** Frontend/design candidates are genuinely expected to have a portfolio because the site *is* a work sample; for backend/systems the screened-for signal is "can you build and reason about real systems," which flows through GitHub, OSS contributions, deployed services, and write-ups — not visual portfolio craft.
- **The dominant hiring factors for interns are elsewhere.** Referrals and the internship pipeline are the heavily-evidenced top drivers; passing the LeetCode-style online assessment is the real big-tech gate; a personal website doesn't enter either of those.
- **But there is a real, narrower upside:** as a *hub* for genuine artifacts (a deployed service + systems write-ups + benchmarks + GitHub links), a site can (a) occasionally trigger inbound or interviewer interest, and (b) since you're building it in Rust/Leptos SSR on Fly.io, **the site is itself a low-level/systems work sample** — which is the one case where building it doubles as the project. That dual-use is the strongest honest justification in this specific case.

Net: **Don't build it for hiring ROI as the primary goal — the evidence doesn't support that.** Build it because (a) the implementation is itself a systems project relevant to the target roles, and (b) it's a cheap hub for higher-ROI artifacts. If the time would otherwise go to referrals, an OSS contribution, or interview prep, those have stronger evidence behind them.

A critical caveat on evidence quality: **no rigorous, controlled study isolates a personal website's causal effect on interview/offer rates.** The closest controlled experiment is about *LinkedIn*, not personal sites. So every "a site helps" claim rests on one small survey plus anecdote, while the "referrals/internships dominate" claims rest on peer-reviewed studies and large first-party datasets. That asymmetry is itself the most important takeaway.

---

## 1. Do recruiters/hiring managers actually look at personal sites for interns?

**They'll glance, but it rarely decides anything — and big-tech pipelines mostly route around it.**

- The one direct survey: ~65% of hiring managers said they'd *definitely* look at an inexperienced candidate's portfolio and ~93% would *likely* look — but ~51% said the candidate's chances are not lower without one, and for ~85% only "slightly lower at most." The author (who runs a React course, a mild bias, and whose sample skews frontend/React) summarized it as managers "look at your website… but don't give a crap." [profy.dev](https://profy.dev/article/portfolio-websites-survey) / [dev.to mirror](https://dev.to/profydev/this-survey-among-60-hiring-managers-reveals-don-t-waste-your-time-on-a-react-portfolio-website-17ge) — *Source quality: medium; the only direct survey on this exact question, but small N and self-selected.*

- Initial resume scans are extremely fast (~6–7 seconds in TheLadders' eye-tracking study), which structurally limits click-through to external sites at the top of the funnel. [TheLadders study PDF](https://www.theladders.com/static/images/basicSite/pdfs/TheLadders-EyeTracking-StudyC2.pdf) / [HR Dive coverage](https://www.hrdive.com/news/eye-tracking-study-shows-recruiters-look-at-resumes-for-7-seconds/541582/) — *Source quality: medium-high for "scans are fast"; the no-click inference is reasonable but indirect.*

- **Big-tech vs. startup split.** Large companies often *don't* review portfolios because their standardized processes apply a "lowest common denominator" (not every applicant has one); smaller/personal companies are more likely to actually look. This comes from a CS professor who advises students and is contacted by recruiters reviewing student portfolios several times a year. [philipmjohnson.org](https://philipmjohnson.org/essays/portfolios-2020.html) — *medium.* At FAANG-style firms the pipeline is resume screen → coding assessment → interviews, with sites secondary at best. [Tech Interview Handbook](https://www.techinterviewhandbook.org/resume/) — *medium-high (consensus practitioner guidance).*

- **The site is read later, not at the top of the funnel.** When it matters, it's in later rounds where you're asked to walk through and justify project decisions — not as a first-pass filter. [Tech Interview Handbook](https://www.techinterviewhandbook.org/software-engineering-interview-guide/) — *medium.*

- **Evidence gaps worth naming:** Neither NACE (the authoritative US new-grad employer survey) nor the Stack Overflow Developer Survey directly measures "do recruiters review personal websites." NACE measures resume attributes; SO measures developer behavior/tools. So the core question genuinely rests on one small survey plus convergent anecdote. [NACE resume attributes](https://www.naceweb.org/about-us/press/the-key-attributes-employers-are-looking-for-on-graduates-resumes) · [Stack Overflow 2024 survey](https://survey.stackoverflow.co/2024/) (absence confirmed by direct fetch) — *high quality sources, but an explicit evidence gap on this question.*

- Much online "portfolio review" data is about **UX/design roles, not SWE** — be cautious importing those numbers.

---

## 2. What specifically moves the needle vs. what is noise or negative

**Positive signals (well-supported by convergent practitioner consensus):**

- **Live, working demos beat code-only repos; quality beats quantity** (≈3–5 polished projects over 10+ basic ones). [CareerFoundry](https://careerfoundry.com/en/blog/web-development/software-engineer-portfolio/) — *low-medium; bootcamp content, the specific percentages are untraceable to a primary source, but the direction is consensus.*
- **Original projects solving a real problem** are a positive signal; tutorial clones (to-do, weather, calculator apps) are noise reviewers skip. [Scrimba](https://v1.scrimba.com/articles/web-developer-portfolio-mistakes/) · [dev.to](https://dev.to/__be2942592/stop-building-portfolio-projects-nobody-cares-about-3h14) — *medium.*
- **Clear READMEs, design notes, tests/CI, readable commit history** differentiate similar-stack candidates by signaling team-readiness; reviewers specifically want to discuss "why this architecture? did it end up being ideal?" [dev.to thread led by DEV founder Ben Halpern](https://dev.to/ben/why-do-employers-check-github-profiles-1fi4) — *medium-high (credible practitioner discussion).*
- **Context over raw artifacts.** One developer's "500+ GitHub stars but zero offers" story illustrates that reviewers (spending ~45–90s) need to see *what you specifically did and the impact*, not just artifacts or star counts. [Medium anecdote](https://medium.com/@alaxhenry0121/my-github-had-500-stars-but-zero-job-offers) — *low-medium; anecdotal, but the principle recurs.*

**Noise (glanced at, rarely decisive):** the portfolio *site as an artifact*, skill-percentage bars ("HTML 90%"), over-design, buzzword soup. [Scrimba](https://v1.scrimba.com/articles/web-developer-portfolio-mistakes/) — *medium.*

**Actively negative:** dead/broken demo links, repos untouched for months (reads as abandonment), fork-only/tutorial-only GitHub, and anything that invites a "did you really build this?" question you can't answer. The survey author explicitly warns a portfolio site "can backfire" via poor design, broken links, or stale content. [profy.dev](https://profy.dev/article/portfolio-websites-survey) · [thisisglance](https://thisisglance.com/blog/mobile-app-developer-portfolio-analysis-decode-the-warning-signs) — *medium / low-medium.*

**On GitHub "green squares":** depth of contributions matters more than the contribution graph; an inactive graph is generally *not* held against you ("If a candidate is not active I don't hold it against them, I just ask different questions"). The graph is also widely criticized as biased — it favors people with free time and OSS-friendly jobs, since most engineers' best work is private. [dev.to/ben thread](https://dev.to/ben/why-do-employers-check-github-profiles-1fi4) — *medium-high.*

**On technical blogging:** genuine but high-variance career leverage. Gergely Orosz effectively "wrote himself a new job" and built the top tech newsletter from writing [Pragmatic Engineer](https://www.pragmaticengineer.com/) — but that's survivorship-heavy. Julia Evans (whose career is visibly built on writing) directly cautions: *"Tons of amazing developers don't have blogs or personal websites at all. I write because it's fun for me."* [Chris Coyier quoting Evans](https://chriscoyier.net/2023/09/06/julia-evans-on-blogging/) — *high (primary/verified).* Patrick McKenzie ("patio11"), another writing-built career, still argues most jobs come via networking/referrals and endorses blogging mainly as a networking adjunct. [kalzumeus](https://www.kalzumeus.com/2011/10/28/dont-call-yourself-a-programmer/) — *high (classic essay, somewhat dated).*

---

## 3. Backend / ML / low-level-systems vs. frontend/design — does a site matter differently?

**Yes, materially differently — and this is the most useful section for this situation.**

- **Frontend/design candidates are genuinely expected to have a portfolio; backend candidates are not.** Portfolio culture arose in *visually-focused* specialties; for a frontend dev the site "demonstrates that you can do the day-to-day tasks you're being hired to do," whereas for a backend dev "the results of our work are less immediately obvious" and **GitHub effectively serves as the portfolio.** An 18-year backend dev reports no employer ever expressed interest in a portfolio. [HN: "Do Back-End Developers Even Have Portfolios?"](https://news.ycombinator.com/item?id=43270627) — *medium-high as practitioner consensus; anecdotal but strongly convergent.* A structural reason: much backend work can't be shown publicly (NDA/proprietary/internal).

- The frontend portfolio is treated as primary evidence — "they want to click on your live websites" and infer your level from it. [greatfrontend](https://www.greatfrontend.com/blog/frontend-developer-portfolio) — *medium-high.* This expectation simply doesn't transfer to backend/systems.

- **What backend roles screen for:** system design, APIs, databases, scalability. One deep, production-like project (e.g., a game server with matchmaking treated as "a compact systems-design interview") beats many toy apps, and a deployed demo beats screenshots. [nucamp](https://www.nucamp.co/blog/top-10-backend-portfolio-projects-for-2026-that-actually-get-you-hired) — *medium on direction, low on specifics (SEO content).*

- **What ML roles screen for:** a well-documented GitHub plus evidence you've **built and deployed models into real applications**; for researchers, **publications** dominate ("a publication at NeurIPS or Nature is a strong signal"). A few **end-to-end, reproducible, deployed** projects beat many Kaggle notebooks — "three end-to-end projects over twenty Kaggle notebooks every single time." [herohunt](https://www.herohunt.ai/blog/the-ultimate-guide-to-recruiting-ai-engineers-and-ai-researchers/) · [interviewnode](https://www.interviewnode.com/post/how-to-build-a-strong-ml-portfolio-projects-github-kaggle-with-example-projects) — *medium-high on the deployed-end-to-end > notebooks theme.*

- **What low-level/systems roles screen for:** the strongest named source here is Glauber Costa (Linux kernel contributor, ScyllaDB/Turso): for systems programming, **meaningful open-source contributions are the primary hiring signal** and "your usual bullet list of technologies mean very little" — the bar is *substantial* contributions to real codebases, not trivial commits or solo toy projects. [Glauber Costa](https://glaubercosta-11125.medium.com/career-advice-for-young-system-programmers-c7443f2d3edf) — *high for systems specifically (strongest named practitioner), though one person's philosophy.* Real systems job postings (e.g., Canonical) list kernel internals and OSS/kernel contributions as qualifications. [Canonical posting](https://canonical.com/careers/5833620) — *medium-high.*

- **Can a site convey backend/ML/systems signal?** Yes — but only as a *container* for those artifacts (project write-ups, architecture docs, benchmarks, links to a live service and repos). Its design quality is largely irrelevant for these roles. Practitioners are openly skeptical it beats GitHub/blog: "highly confident a well-structured portfolio doesn't actually help in today's job search"; "not sure what a portfolio would do differently other than create more work." [HN thread](https://news.ycombinator.com/item?id=43270627) — *medium-high that a site is neutral-to-marginal for backend.*

- **The specific edge here:** because the site is built in Rust/Leptos SSR self-deployed on Fly.io, the *implementation itself* is a low-level/systems work sample (SSR rendering, a Rust web stack, self-managed deployment). For the target roles, this is the rare case where building the site doubles as a legitimate systems project that can be written up and discussed — which is exactly the artifact type these roles screen for. That is the strongest honest pro-build argument, and it's about the *engineering*, not the portfolio.

---

## 4. What actually gets interns hired (honest ROI perspective)

This is where the evidence is *strongest* — and it's not the website.

- **Referrals are the best-documented high-leverage factor.** First-party ATS data (38M+ applications, 93K jobs, 2021–2024): referred candidates advance application→interview at ~40% and ~16% of interviewed referrals get offers, while by 2024 inbound applicants' offer rate fell to ~0.2% — yet referrals are only ~1% of applications. [Ashby Talent Trends](https://www.ashbyhq.com/talent-trends-report/reports/referrals) — *high (large transparent first-party dataset).* Peer-reviewed personnel-economics studies confirm referred applicants are significantly more likely to be hired and to accept, with effects strongest at lower skill levels. [Burks et al., QJE 2015](https://academic.oup.com/qje/article-abstract/130/2/805/2331590) · [Brown et al., J. Labor Econ 2016](https://www.newyorkfed.org/medialibrary/media/research/staff_reports/sr568.pdf) — *high (peer-reviewed).* (Note: the widely-circulated "20% vs 1.2%" and "7%/40%" stats lack clean primary sources — treat as directional only. [Pinpoint](https://www.pinpointhq.com/insights/referrals-are-7x-more-likely-to-be-hired-than-job-board-candidates/))

- **Internship experience is the #1 differentiator between otherwise-equal new grads** (NACE, three+ consecutive years), and tech intern-to-FT conversion is high — overall ~62% offer rate for the 2024 intern class. The pipeline *is* the hiring mechanism. [NACE: internship top differentiator](https://www.naceweb.org/talent-acquisition/candidate-selection/internship-experience-the-top-differentiator-when-choosing-between-otherwise-equal-job-candidates/) · [NACE conversion](https://www.naceweb.org/talent-acquisition/internships/intern-offer-and-conversion-rates-fall-acceptances-rise) — *high.*

- **The LeetCode-style online assessment is the real big-tech gate** — the first hard filter, decided before any human deeply reviews a site. A personal website essentially does not enter this stage. [practitioner write-up](https://hachiyuki8.com/tech/job-hunt-advice) — *medium (practitioner consensus, no formal survey).*

- **GPA screening has collapsed (only ~38% of employers use it)**; employers now prioritize demonstrated skills — problem-solving (~90%), teamwork (~80%), communication, technical skills (≥70%). [NACE GPA/skills](https://www.naceweb.org/talent-acquisition/candidate-selection/as-their-focus-on-gpa-fades-employers-seek-key-skills-on-college-grads-resumes) — *high (stated preferences, not revealed behavior).*

- **GitHub is a secondary "bonus" signal**, weighted below the resume at traditional firms and above it at some OSS-first companies. (The oft-cited "87% of recruiters review GitHub" stat has no traceable primary source — discard it.) [dev.to](https://dev.to/hexshift/what-recruiters-look-for-in-a-github-profile-and-how-to-optimize-yours-j0e) — *low-medium.*

**Synthesized ROI ordering for interns:** (1) referrals + internship/return-offer pipeline → (2) passing the OA/coding-interview gate → (3) demonstrated skills via real projects/GitHub on the resume → far behind, (4) GPA/school, with the **personal website as a low-cost marginal enhancer, not a primary driver.** No study ranks website ROI directly against referrals/OSS — but the *asymmetry of evidence* (referrals strongly proven, website ROI thin/anecdotal) is itself the answer.

---

## 5. Measurable signals to track whether your site is helping

The cleanest candidate-trackable signals (from firsthand accounts):

1. **Count interviews where someone references your site unprompted.** This is the cleanest direct signal — multiple HN engineers report interviewers naming their site as the reason for interest ("At least two of my interviewers this year referenced my website as being why they were interested"). [HN](https://news.ycombinator.com/item?id=41656015) — *anecdotal individually, but the pattern recurs.*

2. **Log inbound recruiter/HM messages and ask where they found you.** Several engineers attribute opportunities to blog/site discovery ("nearly every single professional opportunity… was through my blog" over 9 years). [HN](https://news.ycombinator.com/item?id=41656015) — *firsthand, survivorship-biased.*

3. **Application-to-interview conversion rate, A/B'd with vs. without the site link** — the most rigorous metric you can self-instrument. Baseline reference: ~3% application-to-interview conversion (~33 applications/interview). [Rolebench research](https://www.rolebench.com/research) — *moderate (aggregate, not site-specific).*

4. **Web analytics segmented by referrer** (LinkedIn/direct hits around application time). Available but weak — no source provides a controlled analytics-to-offer conversion figure.

**Strongest adjacent experiment (a caution as much as a hope):** a field experiment with 24,570 fictitious resumes found that including a *comprehensive* LinkedIn profile raised callbacks from 7.9% to 13.5% (a 71% lift), while a *bare-bones* profile slightly *lowered* it (7.2%). [ResumeGo](https://www.resumego.net/research/linkedin-interview-chances/) — *high internal validity, but it tests LinkedIn (not a personal site) and is vendor-run.* The lesson transfers: a thorough online presence can help, a weak/empty one can hurt — so a half-finished site with broken links is worse than none.

**Counter-evidence (negligible impact), kept for honesty:** many engineers report their site did nothing — "all of them claimed they didn't look at my portfolio page"; blogs help only for "a very niche topic" and "GitHub would be a better source." [HN](https://news.ycombinator.com/item?id=41656015) — *anecdotal but an important counterweight to survivorship bias.*

**Flagged as unsubstantiated:** claims like "a portfolio link increases interview invitations by up to 30%" or "48% of hiring managers say a portfolio helped shortlist faster" appear only in SEO/marketing blogs with no underlying study. [resumly.ai](https://www.resumly.ai/blog/how-to-create-a-portfolio-link-that-complements-your-resume) — *very low; do not rely on these.*

---

## Highest-confidence takeaways

1. **The personal website itself is low-ROI for hiring; its contents are what matter.** (Best direct survey + convergent practitioner consensus.) [profy.dev](https://profy.dev/article/portfolio-websites-survey)
2. **Referrals and the internship pipeline are the dominant, best-evidenced drivers** of intern hiring — orders of magnitude better documented than any website effect. [Ashby](https://www.ashbyhq.com/talent-trends-report/reports/referrals), [NACE](https://www.naceweb.org/talent-acquisition/candidate-selection/internship-experience-the-top-differentiator-when-choosing-between-otherwise-equal-job-candidates/), [peer-reviewed studies](https://academic.oup.com/qje/article-abstract/130/2/805/2331590)
3. **For backend/ML/systems, GitHub + deployed services + write-ups + (ML) publications are the screened-for signals; visual portfolio polish is largely irrelevant** — unlike frontend/design, where the site is the work sample. [HN](https://news.ycombinator.com/item?id=43270627), [Glauber Costa](https://glaubercosta-11125.medium.com/career-advice-for-young-system-programmers-c7443f2d3edf)
4. **A weak/broken online presence can actively hurt** — so build it well or not at all. [ResumeGo](https://www.resumego.net/research/linkedin-interview-chances/)
5. **The strongest honest reason to build it:** a Rust/Leptos SSR app self-deployed on Fly.io is *itself* a systems work sample for the target roles — build it as a project and write it up, not as a "portfolio."

## Where the evidence is weak

- **No controlled study isolates a personal website's causal effect on interview/offer rates.** The one rigorous experiment is about LinkedIn. Every "the site helps" claim is one small survey (n≈60, frontend-skewed) plus survivorship-biased anecdote.
- **Neither NACE nor the Stack Overflow Developer Survey directly measures personal-site review** — a real gap in the authoritative datasets.
- **Many circulating stats are untraceable** ("87% review GitHub," "20% vs 1.2% referral hire rate," "30% more interviews from a portfolio") — flagged and not relied upon.
- **Backend/ML/systems "what works" guidance** leans on one strong named practitioner (systems only), a rich-but-anecdotal HN thread, and commercially-motivated SEO blogs that are internally consistent but not authoritative.

**The intellectually honest bottom line:** build the site if you value the *engineering* of building it (and you should, for the target roles) and want a cheap hub for real artifacts — but **do not expect it to be the thing that lands the internship.** Put the marginal hour into a referral, an OSS contribution, or interview/OA prep if hiring outcome is the only goal.

---

# What clients & hiring managers want answered when evaluating a developer

> Research conducted 2026-06-27 to inform **Priority 2 (converting freelance clients)** in [design.md](./design.md).
> Scope: client-side / buyer-side evidence on what people who *hire* a software developer or freelancer look for, and the questions they need answered before reaching out — with emphasis on production-grade/backend work.

## Evidence quality note (read first)

The strongest buyer-side evidence is a small set of surveys; much of the rest is hiring-guide/practitioner opinion. Tags below: **[EMPIRICAL]** = survey/platform data; **[OPINION]** = practitioner/hiring-guide assertion.

- **Clutch** — survey of **312 US professionals who have hired B2B service providers** (fielded late 2022 / early 2023). Genuinely buyer-side, methodology disclosed. [clutch.co/resources/b2b-buyer-data](https://clutch.co/resources/b2b-buyer-data)
- **Influ2** — survey of **50 senior enterprise/mid-market software buyers** (C-level/Director). Small N, explicitly buyer-side. [influ2.com](https://www.influ2.com/blog/enterprise-software-buying-process-survey)
- The popular circulating stats ("87% prefer portfolios to resumes," "55 seconds to decide," "84% want working apps not repos") are **uncited marketing repetition — treat as [OPINION]**, included only as direction.
- Academic anchor for first impressions: Stanford Web Credibility work (Fogg et al.), behind the "~75% judge credibility by design" claim.

## Ranked questions clients/hiring managers want answered

1. **"Do you understand MY specific problem/industry?"** — the single best-evidenced concern. Clutch: understanding the client's specific business need was the **#1 deciding factor (~29%)**, just ahead of raw ability to deliver. Buyers mean a *tailored* solution, not a generic skills list. [clutch.co](https://clutch.co/resources/b2b-buyer-data) **[EMPIRICAL]**
2. **"Can you actually do THIS kind of work?"** — ability to provide the service was the **#2 factor (~28%)**, essentially tied. Buyers look for verified history and prior projects *similar to theirs*, not breadth. [clutch.co](https://clutch.co/resources/b2b-buyer-data) **[EMPIRICAL]** / [index.dev](https://www.index.dev/blog/evaluate-freelance-developer-portfolio) **[OPINION]**
3. **"What did the work actually achieve?"** — buyers want outcomes mapped to value and *storytelling* (problem → decision → result), not feature lists; "missing metrics/impact" is a rejection trigger. [linkedin.com (hiring-manager content)](https://www.linkedin.com/top-content/recruitment-hr/freelance-hiring-solutions/how-hiring-managers-evaluate-freelance-talent/), [index.dev](https://www.index.dev/blog/evaluate-freelance-developer-portfolio) **[OPINION, hiring-manager-sourced]**
4. **"How do you think / make decisions?"** — *how* you approach an unfamiliar problem "matters more than existing knowledge"; walking through tradeoffs and lessons-learned is a top shortlisting signal. [linkedin.com](https://www.linkedin.com/top-content/recruitment-hr/freelance-hiring-solutions/how-hiring-managers-evaluate-freelance-talent/) **[OPINION, hiring-manager-sourced]**
5. **"Will you communicate and deliver reliably — and not disappear?"** — communication/responsiveness is a top non-technical screen; slow/vague response is a leading red flag ("they won't get more responsive after you pay"). [prologica.ai](https://www.prologica.ai/blog/what-are-the-warning-signs-of-a-bad-software-developer) **[OPINION]**
6. **"Is the work credible and low-risk?"** — for technical buyers: viewable **code samples**, **tests + CI**, **README/docs**, sane architecture. Strongest stated red flag: *"if a developer cannot provide code samples, that is a serious warning sign"*; "no testing practices = risky." [index.dev](https://www.index.dev/blog/evaluate-freelance-developer-portfolio), [prologica.ai](https://www.prologica.ai/blog/what-are-the-warning-signs-of-a-bad-software-developer) **[OPINION, hiring-manager-sourced]**
7. **"Can I trust your reputation / what proof exists?"** — reviews matter in B2B: Clutch reports **~47% of businesses read 6+ reviews** before deciding; refusal to give references is a red flag. Precisely the signal a first-timer lacks (see trust-substitutes). [clutch.co](https://clutch.co/resources/b2b-buyer-data) **[EMPIRICAL for review counts]**
8. **"What will it cost / how is payment & risk structured?"** — buyers distrust fixed quotes given before requirements are understood and large upfront payments ("tie payments to milestones"). [prologica.ai](https://www.prologica.ai/blog/what-are-the-warning-signs-of-a-bad-software-developer) **[OPINION]** — *deliberately kept off the site per the show-don't-sell stance; belongs in a first conversation.*
9. **"Are you legit at a glance (professional presentation)?"** — Stanford: **~75% judge credibility by website design**; "sketchy"/no-photo profiles are rejected. [moststudios.com](https://moststudios.com/learn/how-web-design-shapes-user-trust-and-credibility/) **[EMPIRICAL underlying study]** / [upwork.com](https://www.upwork.com/hire/software-developers/) **[OPINION]**

## First-client trust signals (substitutes for testimonials)

For a freelancer with **no testimonials**, consistent (mostly **[OPINION]**) substitutes: a small-but-relevant portfolio (relevance > volume); a visible **diagnosis of the client's problem** (early proof of thinking, converts better than free work); **inspectable artifacts** (code samples, clean GitHub with READMEs/tests/CI, decision write-ups); honesty about scope/limits; professional presentation. One **[EMPIRICAL, small N]** addition: Influ2 found **70% said helpful content addressing real problems is what stands out most** and **64% cited peer word-of-mouth**. [forbesceos.com](https://www.forbesceos.com/how-new-freelancers-can-build-trust-and-credibility-quickly/), [index.dev](https://www.index.dev/blog/evaluate-freelance-developer-portfolio), [influ2.com](https://www.influ2.com/blog/enterprise-software-buying-process-survey)

## Technical/production-grade vs generic-gig differences

- **Generic web/design gigs** are judged heavily on **visual polish** and first-impression design — appearance *is* much of the signal. **[OPINION]**
- **Production-grade backend/systems work** shifts the questions to **operational competence**: monitoring/observability, CI/CD + deploy strategy (blue-green/canary), safe schema migrations, scaling (caching/sharding/load balancing), incident/on-call, IaC/containers. Red flag: *"no monitoring mindset / ignores scale."* [recruiter.daily.dev](https://recruiter.daily.dev/resources/hiring-backend-developer-complete-checklist/), [terminal.io](https://www.terminal.io/blog/15-backend-developer-interview-questions-for-hiring-backend-engineers) **[OPINION]**
- For technical buyers, **code-quality/testing/architecture evidence becomes a gate**; for design gigs it is largely irrelevant.
- Enterprise/production buying adds **security/procurement gatekeepers**: Influ2 — **38% said IT/Security raises the biggest objections**, **20% cited security concerns as a decision delay**. [influ2.com](https://www.influ2.com/blog/enterprise-software-buying-process-survey) **[EMPIRICAL, small N]**
- **Honest caveat:** no controlled study directly compares buyer criteria for "infra/backend" vs "design" freelancers — the contrast is synthesized from separate sources and is directional, not proven.

## Bottom line — core questions to validate the site's information architecture

Ranked by evidence strength: (1) *do you understand my problem?* [EMPIRICAL #1], (2) *can you do THIS work?* [EMPIRICAL #2], (3) *what outcome did it produce?*, (4) *how do you think/decide?*, (5) *is the work credible/low-risk?* (code, tests, docs; for infra: monitoring/deploy/scale/security), (6) *will you communicate/deliver reliably?*, (7) *what proof/reputation exists?* (first-timer substitutes), (8) *cost & risk structure* (kept off-site by design). Answering #1–#6 through the work itself, with #7 via substitutes, maps onto the best-supported buyer concerns. The weakest-evidenced items (specialist-vs-generalist, exact first-impression timing, the uncited "87%/84%" stats) should not be over-weighted.
