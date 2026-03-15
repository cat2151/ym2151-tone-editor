//! Unit tests separated from main source files
//!
//! This module structure allows tests to access private functions
//! while keeping them separate to prevent hallucination issues.

#[cfg(test)]
mod app_tests;

#[cfg(test)]
mod app_ch_param_tests;

#[cfg(test)]
mod app_value_by_tests;

#[cfg(test)]
mod app_adsr_mul_sm_tests;

#[cfg(test)]
mod app_tl_d1l_dt_dt2_tests;

#[cfg(test)]
mod app_ks_ams_tests;

#[cfg(test)]
mod file_ops_tests;

#[cfg(test)]
mod midi_conversion_tests;

#[cfg(test)]
mod register_tests;

#[cfg(test)]
mod register_roundtrip_tests;

#[cfg(test)]
mod ui_tests;

#[cfg(test)]
mod variation_selector_tests;

#[cfg(test)]
mod verbose_logging_tests;

#[cfg(test)]
mod random_tone_tests;

#[cfg(test)]
mod history_tests;
