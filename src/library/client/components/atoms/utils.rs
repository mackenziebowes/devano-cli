use crate::library::client::utils::utils::UiComponent;

#[derive(Debug, Clone, Copy)]
pub enum Utils {
    Cn,
}

/// Provides utility methods for working with UI components.
///
/// # Methods
///
/// * `get_ui` - Returns a reference to a static `UiComponent` associated with the utility.
///
/// # Returns
///
/// A reference to a static `UiComponent` corresponding to the specific utility variant.
impl Utils {
    pub fn get_ui(&self) -> &'static UiComponent {
        match &self {
            Utils::Cn => &CN,
        }
    }
}

pub const CN_LIT: &str = r#"
import { twMerge } from "tailwind-merge";
import clsx from "clsx";
import type { ClassValue } from "clsx";

export const cn = (...inputs: ClassValue[]) => {
	return twMerge(clsx(inputs));
};
"#;

pub const CN: UiComponent = UiComponent {
    name: "cn",
    description: "ClassName utility",
    long_description: "",
    filename: "cn.ts",
    folder_path: "utils",
    contents: CN_LIT,
    npm_deps: &["tailwind-merge", "clsx"],
};
