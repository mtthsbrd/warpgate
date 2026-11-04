## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.

## 2024-10-06 - XSS vulnerabilities in raw HTML and SVG injections
**Vulnerability:** Raw HTML and SVG content was being injected directly into the DOM using Svelte's `{@html ...}` directive without sanitization, posing a Cross-Site Scripting (XSS) risk.
**Learning:** `snarkdown` does not sanitize HTML, and any raw SVG imported must be treated with caution. Svelte's `{@html ...}` directive blindly trusts the input, making it a common vector for XSS if the data source is not fully trusted or sanitized.
**Prevention:** Always sanitize raw HTML or SVG content before injecting it into the DOM. Use `isomorphic-dompurify` (compatible with Svelte's SSR) and configure it appropriately (e.g., `{ USE_PROFILES: { svg: true } }` for SVGs).
