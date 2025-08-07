# Level 1

### Level 3 - This skips level 2 (MD001 violation)

Some content here with a very long line that exceeds the typical line length limit and should trigger MD013 line length violation when the default 80 character limit is enforced.

```
This code block is missing a language tag (MD040 violation)
let x = 42;
```

Here's some **bold text** and *italic text*.

## Another heading

- List item with inconsistent spacing
-  This item has extra space (MD030 violation)
- Back to normal spacing

1. Ordered list
2. Second item  
3. Third item has trailing spaces (MD009 violation)

[Bad link syntax](missing-file.md) - this might trigger MDBOOK002 if in mdBook project

> Blockquote
>   With inconsistent spacing (MD027 violation)

## Duplicate heading

Some content.

## Duplicate heading

This duplicate heading should trigger MD024 violation.