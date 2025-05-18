use crate::library::client::components::features::auth::api_partials::index;
use crate::library::client::components::features::utils::Installable;
use crate::library::client::components::features::writes;
use crate::library::client::utils;
use crate::library::client::utils::utils::ApiComponent;
use anyhow::Result;
use std::fs;
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum AuthRoutes {
    Index,
}

/// Implements the `install` method for the `AuthRoutes` enum.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the installation is successful, or an error if it fails.
///
/// # Behavior
///
/// This method matches on the variant of `AuthRoutes` and performs the corresponding
/// installation logic:
///
/// - For `AuthRoutes::Index`, it calls `writes::add_api` with the `INDEX` constant
///   and propagates any errors that may occur.
///
/// # Errors
///
/// This method propagates any errors returned by `writes::add_api`.
impl Installable for AuthRoutes {
    fn name(&self) -> &str {
        match self {
            AuthRoutes::Index => "auth/index",
        }
    }
    fn install(&self) -> Result<()> {
        match self {
            AuthRoutes::Index => {
                writes::add_api(&index::INDEX)?;
                Ok(())
            }
        }
    }
}
