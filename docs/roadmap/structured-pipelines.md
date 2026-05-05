# R4 structured pipelines design

## Purpose

Traditional shells pass text through pipes. phase1 should support text first, then evolve toward structured command output like a modern terminal OS lab.

The design goal is to keep simple commands familiar while allowing powerful inspection flows.

## Output model

Commands should eventually return:

```rust
enum CommandOutput {
    Text(String),
    Table(Table),
    Json(String),
    Empty,
}
```

Table rows should be internally typed enough for filters:

```rust
struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}
```

## Pipeline syntax

Initial syntax:

```text
ps | table
ps | where state == bg
ps | get pid
ps | sort cpu
ls / | table
```

## Built-in pipeline commands

```text
table   render structured data as aligned columns
json    render structured data as JSON-like text
get     select a column
where   filter rows
sort    sort rows by a column
count   count rows/items
```

## Migration path

1. Add output enum without changing UI behavior.
2. Convert `ps`, `ls`, `jobs`, `ifconfig`, and `audit` to structured outputs internally.
3. Add pipe parser that separates command segments.
4. Add `table` renderer.
5. Add `get`, `where`, `sort`, and `count`.
6. Add smoke tests for deterministic pipelines.

## Expected examples

```text
ps | table
ps | where name == worker
ls / | count
audit | where action == sys.write
ifconfig | get name
```

## Constraints

- Plain command behavior should remain unchanged.
- If a command produces text only, pipeline commands should still work as text filters where possible.
- The parser must keep quoted strings intact.
- Errors should point to the failing pipeline segment.

## Testing targets

- parsing with quoted pipes
- pipeline segment count
- table rendering alignment
- filtering missing columns
- sorting numeric-looking values
