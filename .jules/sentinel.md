## 2024-05-16 - [XSS] Svelte `{@html}` rendering raw untrusted data
**Vulnerability:** Found uses of Svelte's `{@html ...}` rendering untrusted data, specifically markdown parsed with `snarkdown`, warning messages from the backend, and static raw SVGs.
**Learning:** `snarkdown` does not sanitize its output. Rendering its output directly using `{@html}` is a potential XSS vulnerability if the input is untrusted.
**Prevention:** Always sanitize data before injecting it with `{@html}`. In SSR/SvelteKit/Vite contexts, use `isomorphic-dompurify` (not standard `dompurify` as it can crash SSR). Use `DOMPurify.sanitize(data)`. For raw SVGs, use `DOMPurify.sanitize(svg, { USE_PROFILES: { svg: true } })`.
