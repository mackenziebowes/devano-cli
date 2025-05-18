use crate::library::client::components::atoms::registry::Atoms;
use crate::library::client::components::atoms::{
    buttons::Buttons, decorators::Decorators, feedback::Feedbacks, icons::Icons, inputs::Inputs,
    layout::Layouts, utils::Utils,
};
use crate::library::client::components::features::auth::ui_partials::{
    auth, auth_inner, auth_nav, auth_state, log_in_form, register_form,
};
use crate::library::client::components::features::utils::{Installable, MoleculeInstallable};
use crate::library::client::utils::utils::{UiComponent, write_file};
use crate::library::client::{utils, writes};
use anyhow::Result;
use std::fs;
use std::io::{self};
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum AuthMolecules {
    Auth,
    AuthState,
    AuthInner,
    AuthNav,
    LogInForm,
    RegisterForm,
}

/// Implementation of the `AuthMolecules` enum, providing methods for managing
/// UI components and their dependencies in the authentication module.
///
/// # Methods
///
/// ## `atom_dependencies`
/// Returns a static reference to an array of `Atoms` that represent the dependencies
/// required by the specific `AuthMolecules` variant. Each variant has its own set of
/// dependencies, which may include buttons, inputs, or decorators.
///
/// ## `get_ui`
/// Returns a static reference to a `UiComponent` associated with the specific
/// `AuthMolecules` variant. This is used to retrieve the UI component's metadata,
/// such as its folder path, filename, and contents.
///
/// ## `install`
/// Installs the UI component represented by the `AuthMolecules` variant. This includes:
/// - Resolving the base path for the component.
/// - Recursively installing all atom dependencies.
/// - Checking if the required npm dependencies are installed.
/// - Writing the component's file to the appropriate location.
///
/// # Errors
/// The `install` method returns a `Result` that may contain an error if:
/// - A dependency installation fails.
/// - Npm dependencies are not installed.
/// - Writing the component file fails.
impl AuthMolecules {
    pub fn atom_dependencies(&self) -> &'static [Atoms] {
        match self {
            AuthMolecules::Auth => &[],
            AuthMolecules::AuthState => &[],
            AuthMolecules::AuthInner => &[],
            AuthMolecules::AuthNav => &[
                Atoms::Decorators(Decorators::Separators),
                Atoms::Buttons(Buttons::AnchorButton),
            ],
            AuthMolecules::LogInForm => &[
                Atoms::Buttons(Buttons::Button),
                Atoms::Inputs(Inputs::Text),
                Atoms::Inputs(Inputs::Password),
                Atoms::Layout(Layouts::Card),
                Atoms::Feedback(Feedbacks::ErrorMsg),
                Atoms::Decorators(Decorators::Separators),
            ],
            AuthMolecules::RegisterForm => &[
                Atoms::Buttons(Buttons::Button),
                Atoms::Inputs(Inputs::Text),
                Atoms::Inputs(Inputs::Password),
                Atoms::Decorators(Decorators::Separators),
            ],
        }
    }

    pub fn get_ui(&self) -> &'static utils::utils::UiComponent {
        match self {
            AuthMolecules::Auth => &auth::AUTH,
            AuthMolecules::AuthState => &auth_state::AUTH_STATE,
            AuthMolecules::AuthInner => &auth_inner::AUTH_INNER,
            AuthMolecules::AuthNav => &auth_nav::AUTH_NAV,
            AuthMolecules::LogInForm => &log_in_form::LOG_IN_FORM,
            AuthMolecules::RegisterForm => &register_form::REGISTER_FORM,
        }
    }
}

impl Installable for AuthMolecules {
    fn name(&self) -> &str {
        let ui = self.get_ui();
        ui.name
    }

    fn install(&self) -> Result<()> {
        let base_path = Path::new("client/src/devano");
        let ui_component = self.get_ui();
        let component_path = base_path.join(ui_component.folder_path); // Use folder_path

        // Recursively install dependencies
        for dependency in self.atom_dependencies() {
            dependency.install()?; // Recursively call `install` for each dependency
        }

        // // Handle npm dependencies
        writes::npm::check_if_deps_installed(ui_component.npm_deps)?;

        let file_path = component_path.join(ui_component.filename); // Chain folder_path and filename

        write_file(&file_path, ui_component.contents)?;

        Ok(())
    }
}
