//! Unit tests separated from main source files
//! 
//! This module structure allows tests to access private functions
//! while keeping them separate to prevent hallucination issues.

#[cfg(test)]
mod app_tests;

#[cfg(test)]
mod file_ops_tests;

#[cfg(test)]
mod midi_conversion_tests;

#[cfg(test)]
mod register_tests;

#[cfg(test)]
mod ui_tests;
