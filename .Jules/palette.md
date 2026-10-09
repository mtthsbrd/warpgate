## 2024-10-09 - Accessible Icon Buttons in Sveltestrap
**Learning:** Found an accessibility issue pattern across the app's Sveltestrap UI where icon-only `Button` and `DropdownToggle` components using `svelte-fa` lacked ARIA labels.
**Action:** Always verify that interactive elements without visible text have descriptive `aria-label` attributes to ensure screen reader usability.
