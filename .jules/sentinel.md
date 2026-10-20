## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.

## 2024-10-25 - Prevent XSS in Svelte {@html} directive
**Vulnerability:** Raw HTML strings (like SVGs or markdown output) were directly injected into the DOM using Svelte's `{@html}` directive without sanitization, exposing the app to Cross-Site Scripting (XSS).
**Learning:** Svelte's `{@html}` bypasses standard text escaping. If the injected string comes from an external or unsanitized source (like `snarkdown`, which does not sanitize), it can execute arbitrary JavaScript.
**Prevention:** Always use `isomorphic-dompurify` to sanitize HTML (e.g., `{@html DOMPurify.sanitize(html)}`) before passing it to `{@html}`. For SVGs, use `{ USE_PROFILES: { svg: true } }`.
