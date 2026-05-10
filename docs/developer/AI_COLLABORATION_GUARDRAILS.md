# Phase1 AI Collaboration Guardrails

Status: active developer guidance
Scope: AI-assisted Phase1 development, reviews, agent work, and session handoffs

## Purpose

Prevent AI/session/agent drift during Phase1 development.

These guardrails translate observed AI collaboration failure modes into Phase1 project practice. They do not weaken real safety boundaries. They compensate for over-cautious, under-specified, or attribution-drifting development behavior when the project goal is already defined.

## Core Principle

Phase1 development should preserve evidence, source attribution, target scale, and execution clarity.

AI output must distinguish:

- user instruction
- project fact
- observed evidence
- agent inference
- recommendation
- unvalidated hypothesis

Only verified user statements and committed project files may be treated as authoritative instructions.

## Guardrail 1: Source Attribution

Do not convert assistant interpretation into user instruction.

When carrying guidance across sessions, label it by source:

- user quote
- repository file
- test output
- live command output
- assistant inference
- recommendation

Unattributed guidance is treated as drift until verified.

## Guardrail 2: Positive Framing

Prefer direct desired behavior over negative framing.

Use:

- spawn focused agents when the task benefits from parallel review
- validate architecture against the stated target scale
- record evidence and non-claims

Avoid making unwanted behavior the anchor.

## Guardrail 3: Evidence Beats Benchmark Claims

Published metrics, benchmarks, and agent consensus are hypotheses until Phase1 validates them.

A recommendation is not promoted until a live test, repo test, or recorded evidence supports it.

## Guardrail 4: Decision-Time Review

Evaluate exploration at the decision point, not after the outcome looks simple.

Do not label a broad investigation as excessive merely because the final answer is compact.

## Guardrail 5: Target Scale Over Current Scale

Architecture decisions must be evaluated against Phase1 target scale, not only the current demo state.

When the target scale is stated, use that target as the review baseline.

## Guardrail 6: Config Visibility

Tests and validation paths that depend on configuration should expose active config values when useful.

Config drift is treated as a recurring risk during migrations, update paths, model changes, and runtime wiring.

## Guardrail 7: One Change, One Validation

Prefer small reviewable changes.

For risky implementation work:

- one change
- one validation path
- one commit or PR scope

Bundled recommendations remain hypotheses until each part is isolated and validated.

## Guardrail 8: Agent Prompt Self-Sufficiency

Agent prompts must include enough context to stand alone.

Do not assume a fresh agent can see the parent conversation, repository status, or project instructions unless that visibility is verified.

A useful agent prompt should include:

- project name
- current goal
- relevant constraints
- target scale
- exact files or surfaces to inspect
- expected output shape

## Guardrail 9: Bounded Agent Output

Agent tasks should be narrow enough that the result remains useful and compact.

Prefer multiple focused agents over one broad agent that returns oversized, low-signal output.

## Guardrail 10: Test Pass Is Not Quality Certification

Passing tests mean the checked behavior passed.

Do not treat green tests as proof that the whole architecture, all edge cases, or all runtime paths are correct.

## Phase1 Practice

For AI-assisted Phase1 work, preserve the existing project style:

- record validated commands
- record non-claims
- separate planning from promotion
- keep destructive operations out of evidence reports
- prefer dry-run/read-only validation until explicit write paths are reviewed separately

## Non-Claims

- This document does not authorize bypassing safety boundaries.
- This document does not promote installer readiness.
- This document does not promote hardware validation.
- This document does not promote daily-driver readiness.
- This document does not authorize destructive disk writes.
