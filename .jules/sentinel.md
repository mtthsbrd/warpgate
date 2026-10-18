## 2024-05-24 - [XSS] Sanitize HTML rendering in Svelte
**Vulnerability:** XSS vulnerability where raw server inputs (config warnings) and markdown rendering (snarkdown) were passed directly to Svelte's `{@html }` directive without sanitization.
**Learning:** `snarkdown` is used for rendering markdown but does not sanitize HTML, and Svelte's `{@html }` exposes the application to XSS. Also, `isomorphic-dompurify` must be used over `dompurify` to avoid SSR crashes since the Svelte app can be rendered on the server side.
**Prevention:** Always wrap dynamically generated HTML (like markdown output) or external HTML input (like server warnings) with `DOMPurify.sanitize()` from `isomorphic-dompurify` before passing to `{@html }`.
