## 2024-10-05 - Avoid returning internal errors with raw internal messages in HTTP APIs
**Vulnerability:** `poem::Error::from_string(format!("... {e}"))` might leak sensitive information (stack traces, specific DB schemas, proxy targets, network topology) in HTTP error responses returned to clients.
**Learning:** Returning low-level details (like exact `e` or system state) to a client in a 500 error can provide information useful to attackers. A generic error with an internal log is preferred.
**Prevention:** Avoid `format!(..., e)` in `poem::Error::from_string(..., INTERNAL_SERVER_ERROR)`. Log the detailed error instead and return a sanitized HTTP message to the client.

## 2026-11-07 - [XSS mitigation for snarkdown output]
**Vulnerability:** XSS vulnerability through markdown rendering in ProtocolDocs.svelte using the un-sanitized output from `snarkdown`.
**Learning:** `snarkdown` output is not inherently safe and must be sanitized.
**Prevention:** Sanitize the output from `snarkdown` using `isomorphic-dompurify` before feeding it to Svelte's `{@html}` tag.
