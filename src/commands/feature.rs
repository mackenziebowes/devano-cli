use crate::library::client::components::features::registry::Features;
use anyhow::Result;
use cliclack::{intro, outro, select};
use strum::IntoEnumIterator;

pub fn guided_ui() -> Result<()> {
    // todo:
    // split guide between CSS operations (modify/generate/install css theme tokens)
    // and module additions (CMS, CRM, Chat, Forum, etc)
    intro("Devano Features".to_string())?;
    let choices: Vec<_> = Features::iter()
        .map(|feature| {
            (
                feature.name().to_string(),
                feature.name().to_string(),
                feature.description().to_string(),
            )
        })
        .collect();
    let choice = select("What would you like to do?")
        .items(&choices)
        .interact()?;
    if let Some(selected_feature) = Features::iter().find(|feature| feature.name() == choice) {
        selected_feature.install()?;
    } else {
        unreachable!();
    }
    outro("Installed".to_string())?;
    Ok(())
}
