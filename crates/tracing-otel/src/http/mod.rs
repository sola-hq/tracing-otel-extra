//! HTTP tracing utilities for OpenTelemetry integration.
//!
//! This module provides utilities for extracting and managing trace context
//! in HTTP requests and responses.

#[cfg(feature = "context")]
pub mod context;
#[cfg(feature = "fields")]
pub mod fields;
#[cfg(feature = "http")]
pub mod propagation;
#[cfg(feature = "span")]
pub mod span;
