#![allow(dead_code)]
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub enum Severity {
  Allow,
  Warning,
  Error,
}

#[derive(Debug, Clone)]
pub enum WarningType {
  UnusedVariable,
  UnusedFunction,
}

#[derive(Debug, Clone)]
pub enum DiagnosticOrigin {
  Compiler,
  VirtualMachine,
  Builtin,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
  pub message: String,
  pub severity: Severity,
}

#[derive(Debug, Clone)]
pub struct Diagnostics {
  pub error_count: usize,
  pub diagnostics: BTreeMap<DiagnosticOrigin, Vec<Diagnostic>>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticsConfig {
  pub report_unused_variables: Severity,
  pub report_unused_functions: Severity,
}
