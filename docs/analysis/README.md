# Phase1 analysis track

This directory tracks Phase1's Program Loading + Analysis work.

The analysis track is intentionally conservative. The first implementation path is static, metadata-oriented, deterministic, and no-execute by default.

Start here:

- [`PROGRAM_LOADING_ANALYSIS.md`](PROGRAM_LOADING_ANALYSIS.md) — workflow, roadmap path, command vocabulary, status plan, and safety boundary.

## Boundary

Phase1 analysis work does not currently claim hardened malware sandboxing, VM/container isolation, production forensic admissibility, or safe execution of hostile binaries.

The initial target is controlled loading of a program-like file for diagnostics, dissection, examination, analysis, and forensic review without executing the sample.
