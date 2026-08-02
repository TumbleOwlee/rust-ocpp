/// Validate implementation using official json schemas
/// provided by the Open Charge Alliance
#[cfg(test)]
mod schema_validation;

/// Tests for the generated `helpers` modules, which cannot carry their own
/// `#[cfg(test)] mod tests` because they are overwritten on regeneration.
#[cfg(test)]
mod helpers;
