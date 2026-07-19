# TODO

- Soft assertions (collect-all-failures mode): currently every assertion
  in a chain panics on first failure. Add an opt-in mode that collects
  all failures in a chain and reports them together at the end, cutting
  round-trips for both humans and AI agents iterating on fixes. Sketched
  (not yet designed) in
  `docs/superpowers/specs/2026-07-19-structural-diff-and-soft-assertions-design.md`
  — needs its own brainstorming pass before a spec/plan.
