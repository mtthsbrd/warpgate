## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.
## 2024-10-06 - XSS vulnerability in snarkdown usage and raw SVG usage via Svelte {@html} directive
**Vulnerability:** Use of Svelte `{@html ...}` directive injecting un-sanitized markdown html using snarkdown, and un-sanitized raw SVG images.
**Learning:** `snarkdown` is a fast markdown parser but it does not sanitize user input. Furthermore, raw SVGs may contain scripting.
**Prevention:** Always wrap variables passed into `{@html ...}` with a sanitizer such as `DOMPurify.sanitize(...)`. When sanitizing SVG strings via DOMPurify, use `{ USE_PROFILES: { svg: true } }` so the image does not get stripped. Use `isomorphic-dompurify` for DOMPurify to work safely during SSR in Svelte.
