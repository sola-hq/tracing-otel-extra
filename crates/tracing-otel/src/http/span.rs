//! HTTP request span creation utilities.

use crate::{
    dyn_span,
    http::{context, fields},
};
use http::Request;
use tracing::{Level, Span, field::Empty};

/// Creates a new [`Span`] for the given request.
pub fn make_request_span<B>(level: Level, request: &Request<B>) -> Span {
    let span = dyn_span!(
        level,
        "request",
        // HTTP fields
        http.version = ?fields::extract_http_version(request),
        http.host = ?fields::extract_host(request),
        http.method = ?fields::extract_http_method(request),
        http.route = Empty,
        http.scheme = ?fields::extract_http_scheme(request).map(debug),
        http.status = Empty,
        http.target = ?fields::extract_http_target(request),
        http.user_agent = ?fields::extract_user_agent(request),
        // OpenTelemetry fields
        otel.name = Empty,
        otel.kind = ?Empty,
        otel.status = Empty,
        // Request tracking
        request_id = %fields::extract_request_id(request),
        trace_id = Empty
    );
    context::set_otel_parent(request.headers(), &span);
    span
}
