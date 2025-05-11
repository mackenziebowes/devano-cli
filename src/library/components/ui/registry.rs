use std::collections::HashMap;

use super::buttons;
use super::inputs;
use super::utils;

pub fn make_component_registry() -> HashMap<&'static str, &'static utils::UiComponent> {
    let mut map = HashMap::new();
    map.insert(buttons::BUTTON.name, &buttons::BUTTON);
    map.insert(inputs::TEXT_INPUT.name, &inputs::TEXT_INPUT);

    map
}