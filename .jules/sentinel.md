## 2024-05-24 - Cross-Site Scripting (XSS) in Config Warnings and Protocol Docs
**Vulnerability:** Unsanitized variables `warning` (from `$serverInfo.configWarnings`) and `html` (from `protocolInfo`) were rendered using Svelte's `{@html ...}` directive, potentially exposing the application to Cross-Site Scripting (XSS) if those variables contain malicious HTML.
**Learning:** Svelte's `{@html}` directive renders content as-is and does not perform any sanitization. Therefore, it is unsafe to use it directly with data that might be controlled or influenced by external sources (like config files or API responses), even if those sources are assumed to be safe.
**Prevention:** Always sanitize data before using `{@html}`. In this case, `DOMPurify` was introduced and utilized to sanitize the variables before rendering.

## 2024-05-24 - Cross-Site Scripting (XSS) in Config Warnings and Protocol Docs
**Vulnerability:** Unsanitized variables `warning` (from `$serverInfo.configWarnings`) and `html` (from `protocolInfo`) were rendered using Svelte's `{@html ...}` directive, potentially exposing the application to Cross-Site Scripting (XSS) if those variables contain malicious HTML.
**Learning:** Svelte's `{@html}` directive renders content as-is and does not perform any sanitization. Therefore, it is unsafe to use it directly with data that might be controlled or influenced by external sources (like config files or API responses), even if those sources are assumed to be safe.
**Prevention:** Always sanitize data before using `{@html}`. In this case, `DOMPurify` was introduced and utilized to sanitize the variables before rendering.
