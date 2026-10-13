## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.

## 2024-10-06 - Prevent XSS in Svelte `{@html ...}` directives using DOMPurify
**Vulnerability:** Unsanitized raw HTML (e.g., from server warnings or SVG raw imports) or markdown rendered with `snarkdown` is injected into the DOM using Svelte's `{@html ...}` directive, leading to potential Cross-Site Scripting (XSS) vulnerabilities.
**Learning:** `snarkdown` does not sanitize HTML, and Svelte's `{@html ...}` directive trusts the provided input completely. Any untrusted or external input must be sanitized to prevent malicious script execution.
**Prevention:** Always sanitize raw HTML and markdown output with `DOMPurify.sanitize()` before injecting it via the `{@html ...}` directive in Svelte.
