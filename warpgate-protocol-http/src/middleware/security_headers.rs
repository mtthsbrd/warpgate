use poem::http::{HeaderValue, header};
use poem::{Endpoint, IntoResponse, Middleware, Request, Response};
pub use warpgate_common_http::{WARPGATE_CSP, WARPGATE_PLAYGROUND_CSP};

#[derive(Clone)]
pub struct ContentSecurityPolicyMiddleware;

impl<E: Endpoint> Middleware<E> for ContentSecurityPolicyMiddleware {
    type Output = ContentSecurityPolicyEndpoint<E>;

    fn transform(&self, inner: E) -> Self::Output {
        ContentSecurityPolicyEndpoint { inner }
    }
}

pub struct ContentSecurityPolicyEndpoint<E: Endpoint> {
    inner: E,
}

impl<E: Endpoint> Endpoint for ContentSecurityPolicyEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let mut resp = self.inner.call(req).await?.into_response();
        if !resp.headers().contains_key(header::CONTENT_SECURITY_POLICY) {
            resp.headers_mut().insert(
                header::CONTENT_SECURITY_POLICY,
                HeaderValue::from_static(WARPGATE_CSP),
            );
        }
        // Prevent Clickjacking attacks
        if !resp.headers().contains_key(header::X_FRAME_OPTIONS) {
            resp.headers_mut().insert(
                header::X_FRAME_OPTIONS,
                HeaderValue::from_static("SAMEORIGIN"),
            );
        }
        // Prevent MIME type sniffing
        if !resp.headers().contains_key(header::X_CONTENT_TYPE_OPTIONS) {
            resp.headers_mut().insert(
                header::X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("nosniff"),
            );
        }
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use poem::endpoint::make_sync;
    use poem::{EndpointExt, Request, Response};

    use super::*;

    #[tokio::test]
    async fn adds_strict_csp_when_absent() {
        let ep = make_sync(|_| Response::builder().finish()).with(ContentSecurityPolicyMiddleware);
        let resp = ep.call(Request::default()).await.unwrap();
        assert_eq!(
            resp.headers().get(header::CONTENT_SECURITY_POLICY).unwrap(),
            WARPGATE_CSP
        );
        assert_eq!(
            resp.headers().get(header::X_FRAME_OPTIONS).unwrap(),
            "SAMEORIGIN"
        );
        assert_eq!(
            resp.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(),
            "nosniff"
        );
    }

    #[tokio::test]
    async fn preserves_existing_csp() {
        // Endpoints such as the OpenAPI playground set their own relaxed policy,
        // which must not be overwritten by the strict default.
        let ep = make_sync(|_| {
            Response::builder()
                .header(header::CONTENT_SECURITY_POLICY, WARPGATE_PLAYGROUND_CSP)
                .finish()
        })
        .with(ContentSecurityPolicyMiddleware);
        let resp = ep.call(Request::default()).await.unwrap();
        assert_eq!(
            resp.headers().get(header::CONTENT_SECURITY_POLICY).unwrap(),
            WARPGATE_PLAYGROUND_CSP
        );
    }
}
