# Walkthrough: GitHub Issues Reorganization

## Summary
Complete reorganization of the S.H.I.F.T. GitHub repo's issue tracking system. Transformed 75 open issues from a flat, inconsistently-labeled list into a fully structured, multi-axis filterable system with milestones, templates, cross-references, and pinned issues.

---

## What Changed

### 1. Label Taxonomy (32 labels)

**Deleted** 20 old labels (7 unused GitHub defaults + 13 flat labels).
**Created** 32 new labels across 7 axes:

| Axis | Labels | Purpose |
|------|--------|---------|
| **Type** | `type: bug`, `type: feature`, `type: security`, `type: performance`, `type: cleanup`, `type: research`, `type: epic` | What kind of work |
| **Priority** | `P0: critical`, `P1: high`, `P2: medium`, `P3: low` | How urgent |
| **Component** | `component: tee-vault`, `component: zk-engine`, `component: p2p-mesh`, `component: block-lattice`, `component: android-app`, `component: pricing-engine`, `component: devops`, `component: identity` | What subsystem |
| **Phase** | `phase: 1` through `phase: 5` | Which development phase |
| **Status** | `status: blocked`, `status: in-progress`, `status: needs-design` | Workflow state |
| **Platform** | `platform: android`, `platform: ios` | Target platform |
| **Language** | `lang: rust`, `lang: kotlin` | Code language |
| **Special** | `audit` | Kept from before |

**Key filter queries you can now run:**
- All critical bugs: `label:"P0: critical" label:"type: bug"`
- All ZK engine work: `label:"component: zk-engine"`
- All security issues: `label:"type: security"`
- All audit findings: `label:audit`
- Blocked items: `label:"status: blocked"`

---

### 2. Milestones (6 milestones, 72 issues assigned)

| Milestone | Due Date | Issues | Description |
|-----------|----------|--------|-------------|
| **M0: Audit Fixes** | Jul 10, 2026 | 21 | All 20 audit issues (A1-A20) + checklist tracker |
| **M1: Root of Trust** | Oct 2, 2026 | 13 | Phase 1 — TEE, PoL, identity, ZK distance bounding |
| **M2: P2P Mesh MVP** | Dec 25, 2026 | 18 | Phase 2 — Mesh networking, dead zones, state channels |
| **M3: Ledger & Settlement** | Mar 5, 2027 | 8 | Phase 3 — Block-Lattice ops, Verkle trees, Nova |
| **M4: Economics & AI** | May 14, 2027 | 7 | Phase 4 — Pricing, slashing, zkML |
| **M5: Production UX** | Jul 9, 2027 | 5 | Phase 5 — SOS, disputes, Web2 UX |

**Weekly schedule:** ~15 hrs/week, see [#117](https://github.com/Keshuin0/S.H.I.F.T/issues/117) for the detailed breakdown.

---

### 3. Issue Templates (3 files)

| File | Purpose |
|------|---------|
| [bug_report.yml](file:///D:/Project/Project%20S.H.I.F.T/.github/ISSUE_TEMPLATE/bug_report.yml) | Structured bug reports with severity, component, reproduction steps |
| [feature_request.yml](file:///D:/Project/Project%20S.H.I.F.T/.github/ISSUE_TEMPLATE/feature_request.yml) | Feature requests with acceptance criteria, phase, component |
| [config.yml](file:///D:/Project/Project%20S.H.I.F.T/.github/ISSUE_TEMPLATE/config.yml) | Disables blank issues, forces template usage |

---

### 4. Relabeling (75 issues updated)

Every open issue was relabeled with the new multi-axis taxonomy:
- Phase 1: 13 issues (5 epics + 8 tasks)
- Phase 2: 19 issues (4 epics + 15 tasks)
- Phase 3: 8 issues (2 epics + 6 tasks)
- Phase 4: 7 issues (2 epics + 5 tasks)
- Phase 5: 5 issues (1 epic + 4 tasks)
- DevOps: 3 issues
- Audit: 20 issues

**Priority distribution:**
- P0 Critical: 6 issues
- P1 High: 27 issues
- P2 Medium: 17 issues
- P3 Low: 10 issues

---

### 5. Cross-References (12 audit issues linked)

All critical and major audit issues now have a `### Dependencies` section with:
- **Blocks:** Issues that can't progress until this is fixed
- **Blocked by:** Issues that must be fixed first
- **Related:** Conceptually linked issues
- **Tracked in:** Points to #118 (Audit Fixes Checklist)

Key dependency chains documented:
```
A13 → A10 → #51 (BLE Beacon)
A11 → #90 (SELinux) + #87 (pKVM)
A1 → #57 (Block-Lattice) + #29 (SBT)
A2 + A5 → #89 (ZK-SNARK for ToF)
A4 → #42 (DHT) → #44 (Proximity)
A7 → #34 (Radio Mesh)
```

---

### 6. Pinned Issues (3 issues pinned)

| Issue | Purpose |
|-------|---------|
| [#117 — Roadmap and Progress Tracker](https://github.com/Keshuin0/S.H.I.F.T/issues/117) | Master checklist of all phases, milestones, weekly schedule |
| [#118 — M0: Audit Fixes Checklist](https://github.com/Keshuin0/S.H.I.F.T/issues/118) | Consolidated checklist of all 20 audit issues with dependency graph |
| [#1 — Phase 1 Epic](https://github.com/Keshuin0/S.H.I.F.T/issues/1) | Active development phase |

---

## Files Changed

| File | Action |
|------|--------|
| `.github/ISSUE_TEMPLATE/bug_report.yml` | **NEW** — Structured bug report template |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | **NEW** — Feature request template |
| `.github/ISSUE_TEMPLATE/config.yml` | **NEW** — Disable blank issues |

Committed and pushed to `dev` branch.

---

## Verification Results

All filters confirmed working:
- ✅ Priority labels filter correctly (P0=6, P1=27, P2=17, P3=10)
- ✅ Component labels filter correctly (8 components, all populated)
- ✅ Milestones show correct counts (M0=21, M1=13, M2=18, M3=8, M4=7, M5=5)
- ✅ Pinned issues visible at top of issues page
- ✅ Issue templates pushed to repo
- ✅ Cross-references render as clickable links in issue bodies
