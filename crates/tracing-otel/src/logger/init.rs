//! Logger initialization functions.

use crate::otel::OtelGuard;
use anyhow::{Context, Result};

use super::config::Logger;
use super::subscriber::{create_output_layers, setup_tracing};

/// Initialize tracing from a Logger configuration
pub fn init_tracing_from_logger(logger: Logger) -> Result<OtelGuard> {
    let layers = create_output_layers(&logger)?;
    let guard = setup_tracing(
        &logger.service_name,
        &logger.attributes,
        logger.sample_ratio,
        logger.metrics_interval_secs,
        logger.level,
        layers,
        logger.otel_logs_enabled,
    )
    .context("Failed to initialize tracing")?;
    Ok(guard)
}

/// Convenience function to initialize tracing with default settings
pub fn init_logging(service_name: &str) -> Result<OtelGuard> {
    let logger = Logger::new(service_name);
    init_tracing_from_logger(logger)
}
