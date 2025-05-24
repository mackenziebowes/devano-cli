use super::{
    buttons::Buttons, decorators::Decorators, feedback::Feedbacks, icons::Icons, inputs::Inputs,
    layout::Layouts, utils::Utils,
};
use crate::library::client::{utils, writes};
use anyhow::Result;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum Atoms {
    /// A collection of button components.
    Buttons(Buttons),
    /// A collection of decorators for modifying components.
    Decorators(Decorators),
    /// Feedback information encapsulated in `Feedbacks`.
    Feedback(Feedbacks),
    /// A collection of reusable icons.
    Icons(Icons),
    /// Input-related components and functionality.
    Inputs(Inputs),
    /// Layout configurations for structuring components.
    Layout(Layouts),
    /// Utility functions and helpers.
    Utils(Utils),
}

/// The `Atoms` implementation provides methods to manage and interact with UI components
/// and their dependencies in the `devano` project. It includes functionality for retrieving
/// dependencies, accessing UI component details, and installing components.
///
/// # Methods
///
/// ## `devano_dependencies`
///
/// Returns a static slice of `Atoms` representing the dependencies required by the current
/// `Atoms` instance. The dependencies are determined based on the specific variant of `Atoms`.
///
/// - **Returns:**  
///   A static slice of `Atoms` dependencies.
///
/// ## `get_ui`
///
/// Retrieves the `UiComponent` associated with the current `Atoms` instance. This method
/// delegates the responsibility to the specific variant of `Atoms` to provide its UI component.
///
/// - **Returns:**  
///   A reference to a `UiComponent` instance.
///
/// ## `install`
///
/// Installs the current `Atoms` instance by performing the following steps:
/// 1. Resolves the base path for the component installation.
/// 2. Retrieves the associated `UiComponent` details.
/// 3. Recursively installs all dependencies of the current `Atoms` instance.
/// 4. Ensures that required npm dependencies are installed.
/// 5. Writes the component's contents to the appropriate file path.
///
/// - **Returns:**  
///   A `Result` indicating success or failure of the installation process.
///
/// - **Errors:**  
///   This method propagates errors that may occur during dependency installation,
///   npm dependency checks, or file writing operations.
impl Atoms {
    pub fn all_variants() -> &'static [Atoms] {
        &[
            Atoms::Buttons(Buttons::Anchor),
            Atoms::Buttons(Buttons::AnchorButton),
            Atoms::Buttons(Buttons::Button),
            Atoms::Buttons(Buttons::ButtonAnchor),
            Atoms::Buttons(Buttons::IconButton),
            Atoms::Decorators(Decorators::Separators),
            Atoms::Feedback(Feedbacks::ErrorMsg),
            Atoms::Icons(Icons::EyeClosed),
            Atoms::Icons(Icons::EyeOpen),
            Atoms::Inputs(Inputs::Text),
            Atoms::Inputs(Inputs::Password),
            Atoms::Inputs(Inputs::Otp),
            Atoms::Layout(Layouts::Card),
            Atoms::Layout(Layouts::Page),
            Atoms::Layout(Layouts::PageInner),
            Atoms::Layout(Layouts::Stack),
            Atoms::Layout(Layouts::Heading),
            Atoms::Layout(Layouts::Modal),
            Atoms::Utils(Utils::Cn),
        ]
    }

    pub fn devano_dependencies(&self) -> &'static [Atoms] {
        match self {
            Atoms::Buttons(button) => match button {
                Buttons::Anchor => &[Atoms::Utils(Utils::Cn)],
                Buttons::AnchorButton => &[Atoms::Utils(Utils::Cn)],
                Buttons::Button => &[Atoms::Utils(Utils::Cn)],
                Buttons::ButtonAnchor => &[Atoms::Utils(Utils::Cn)],
                Buttons::IconButton => &[Atoms::Utils(Utils::Cn)],
            },
            Atoms::Decorators(decorator) => match decorator {
                Decorators::Separators => &[Atoms::Utils(Utils::Cn)],
            },
            Atoms::Feedback(feedback) => match feedback {
                Feedbacks::ErrorMsg => &[Atoms::Utils(Utils::Cn)],
            },
            Atoms::Icons(icon) => match icon {
                Icons::EyeClosed => &[Atoms::Utils(Utils::Cn)],
                Icons::EyeOpen => &[Atoms::Utils(Utils::Cn)],
            },
            Atoms::Inputs(input) => match input {
                Inputs::Text => &[Atoms::Utils(Utils::Cn)],
                Inputs::Password => &[
                    Atoms::Utils(Utils::Cn),
                    Atoms::Icons(Icons::EyeClosed),
                    Atoms::Icons(Icons::EyeOpen),
                    Atoms::Buttons(Buttons::IconButton),
                ],
                Inputs::Otp => &[Atoms::Utils(Utils::Cn)],
            },
            Atoms::Layout(layout) => match layout {
                Layouts::Card => &[Atoms::Utils(Utils::Cn)],
                Layouts::Page => &[],
                Layouts::PageInner => &[Atoms::Utils(Utils::Cn)],
                Layouts::Stack => &[Atoms::Utils(Utils::Cn)],
                Layouts::Heading => &[Atoms::Utils(Utils::Cn)],
                Layouts::Modal => &[],
            },
            Atoms::Utils(util) => match util {
                Utils::Cn => &[],
            },
        }
    }

    pub fn get_ui(&self) -> &'static utils::utils::UiComponent {
        match self {
            Atoms::Buttons(button) => button.get_ui(),
            Atoms::Decorators(decorators) => decorators.get_ui(),
            Atoms::Feedback(feedbacks) => feedbacks.get_ui(),
            Atoms::Icons(icon) => icon.get_ui(),
            Atoms::Inputs(input) => input.get_ui(),
            Atoms::Layout(layouts) => layouts.get_ui(),
            Atoms::Utils(utils) => utils.get_ui(),
        }
    }

    pub fn install(&self) -> Result<()> {
        let base_path = Path::new("client/src/devano");
        let ui_component = self.get_ui();
        let component_path = base_path.join(ui_component.folder_path); // Use folder_path

        // Recursively install dependencies
        for dependency in self.devano_dependencies() {
            dependency.install()?; // Recursively call `install` for each dependency
        }

        let file_path = component_path.join(ui_component.filename); // Chain folder_path and filename

        utils::utils::write_file(&file_path, ui_component.contents)?;

        Ok(())
    }

    pub fn install_all() -> Result<()> {
        for atom in Atoms::all_variants() {
            atom.install()?;
        }
        Ok(())
    }
}
