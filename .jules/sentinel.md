## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.
## 2024-05-24 - [Fix XSS vulnerability in Svelte {@html} tag]
**Vulnerability:** Found `{@html}` tags rendering untrusted input (e.g. from Markdown via `snarkdown` and server warnings) without prior sanitization, creating an XSS risk.
**Learning:** Svelte's `{@html}` directive doesn't perform any sanitization. Furthermore, when using Svelte SSR (Server-Side Rendering), the standard `dompurify` package crashes because it expects `window` to be defined. We must use `isomorphic-dompurify`.
**Prevention:** Always wrap variables passed to `{@html}` with `DOMPurify.sanitize()`, and use `isomorphic-dompurify` instead of `dompurify` in Svelte/SSR environments to prevent server crashes.
