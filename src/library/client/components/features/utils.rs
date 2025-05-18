use crate::library::client::components::atoms::registry::Atoms;
use crate::library::client::utils::utils::{UiComponent, write_file};
use anyhow::Result;

pub trait Installable {
    fn name(&self) -> &str;
    fn install(&self) -> Result<()>;
}

pub trait MoleculeInstallable: Installable {
    fn atom_dependencies(&self) -> &'static [Atoms];
    fn get_ui(&self) -> &'static UiComponent;
}
