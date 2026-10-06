## 2024-10-06 - Missing ARIA Labels in Iterative Lists
**Learning:** Actions within iterative lists (like `<Button>` arrows and `<DropdownToggle>` ellipses in `TargetList.svelte`) frequently miss `aria-label`s since they do not inherently contain descriptive text. This is a common pattern that negatively affects screen reader accessibility.
**Action:** Always check interactive elements that do not contain visible descriptive text, specifically those in repeated rows or iterative lists, and assign descriptive `aria-label`s.
