use crate::library::client::components::features::utils::Installable;
use crate::library::client::components::features::{
    auth::api::AuthRoutes, auth::ui::AuthMolecules,
};
use crate::library::client::utils::utils;
use anyhow::Result;
use cliclack::{intro, multi_progress, outro, progress_bar};
use std::thread;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Features {
    Auth,
}

/// Represents a collection of features and provides methods to handle their dependencies
/// and installation.
///
/// # Methods
///
/// - `feature_dependencies`: Returns a static reference to an array of feature dependencies
///   for the current feature. This helps in determining which features are required
///   before installing the current feature.
///
/// - `install`: Installs the current feature by iterating over its associated molecules
///   and routes, invoking their respective installation methods. Returns a `Result`
///   indicating success or failure of the installation process.
///
/// # Example
///
/// ```rust
/// let feature = Features::Auth;
/// let dependencies = feature.feature_dependencies();
/// feature.install()?;
/// ```
impl Features {
    pub fn feature_dependencies(&self) -> &'static [Features] {
        match self {
            Features::Auth => &[],
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Features::Auth => "Auth",
        }
    }
    pub fn description(&self) -> &str {
        match self {
            Features::Auth => "The foundational module - login/signup/etc.",
        }
    }
    pub fn install(&self) -> Result<()> {
        match self {
            Features::Auth => {
                intro("Installing Auth")?;
                let multi = multi_progress("Cloning...");
                let all_molecules = AuthMolecules::iter();
                let all_routes = AuthRoutes::iter();
                let pb_m = multi.add(progress_bar(all_molecules.len() as u64));
                let pb_r = multi.add(progress_bar(all_routes.len() as u64));
                pb_m.start("Adding components...");
                pb_r.start("Adding routes...");
                let molecule_thread = thread::spawn(move || -> Result<()> {
                    for molecule in all_molecules {
                        pb_m.set_message(format!("Writing: {}", molecule.name()));
                        molecule.install()?;
                        pb_m.inc(1);
                    }
                    pb_m.stop("");
                    pb_m.clear();
                    Ok(())
                });
                let route_thread = thread::spawn(move || -> Result<()> {
                    for route in all_routes {
                        pb_r.set_message(format!("Writing: {}", route.name()));
                        route.install()?;
                        pb_r.inc(1);
                    }
                    pb_r.stop("");
                    pb_r.clear();
                    Ok(())
                });
                molecule_thread.join().unwrap()?;
                route_thread.join().unwrap()?;
                outro("Auth Installed!")?;
                Ok(())
            }
        }
    }
}
