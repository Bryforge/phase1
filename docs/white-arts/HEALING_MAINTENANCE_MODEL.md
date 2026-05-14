# White Arts healing and maintenance model

White Arts healing means staged defensive repair planning. It does not mean silent self-modification or autonomous unsafe mutation.

## Healing principles

- diagnose before repair
- explain findings before proposing changes
- prefer read-only checks
- generate branch-local candidate patches only after explicit operator intent
- require review before promotion
- preserve recovery material
- preserve safe-mode and host-trust boundaries
- never request or store credentials

## Allowed first-stage behavior

White Arts may:

- detect missing docs, links, scripts, tests, metadata, or reports
- explain likely causes
- recommend commands to validate or repair
- generate candidate patches on a branch
- write a report template
- mark blocked claims
- suggest rollback or recovery steps

## Forbidden behavior

White Arts must not:

- silently modify tracked files
- mutate the host system without explicit gates
- delete recovery or evidence files
- collect credentials, tokens, cookies, keys, or recovery codes
- bypass safe-mode or trust gates
- auto-merge repair branches
- claim malware safety or forensic admissibility
- hide failed checks behind optimistic status text

## Maintenance workflow

```text
observe -> classify -> explain -> plan -> stage candidate -> validate -> review -> promote or discard
```

## Candidate repair report

A repair plan should include:

```text
finding id
system
severity
observed evidence
likely cause
proposed repair
files affected
commands to validate
rollback path
claim boundary
review status
```

## Severity labels

```text
info       : useful observation
warning    : non-blocking issue
blocked    : promotion cannot proceed
critical   : safety/security boundary issue
unknown    : evidence is missing
```

## Safety defaults

The default White Arts posture is:

```text
mutation         : none
repair-policy    : staged-candidate-only
host-execution   : disabled by default
review           : required
rollback         : required for promoted repair
```

## Non-claims

A healing recommendation is not proof of correctness. A candidate repair is not release-ready until tests, docs, validation, and review support promotion.
