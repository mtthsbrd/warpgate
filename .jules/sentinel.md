## 2024-10-18 - XSS via Svelte `{@html}` Injection
**Vulnerability:** Raw HTML injection using Svelte's `{@html}` directive with unsanitized output (e.g. raw SVGs, `snarkdown` markdown conversions, config warnings).
**Learning:** Functions like `snarkdown` and raw SVG assets do not perform HTML sanitization. In a single-page app (like Svelte), injecting user-supplied or partially uncontrolled content via `{@html}` can lead to XSS. Note: use `isomorphic-dompurify` rather than `dompurify` to prevent SSR issues and type definition problems.
**Prevention:** Always sanitize any dynamic or externally-sourced HTML before using `{@html}`. Wrap the content with `DOMPurify.sanitize(...)` (using `import DOMPurify from 'isomorphic-dompurify'`), and pass `{ USE_PROFILES: { svg: true } }` if injecting SVGs.

## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.
