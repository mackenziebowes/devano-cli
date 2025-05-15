use crate::library::client::utils::utils::{UiComponent};

#[derive(Debug, Clone, Copy)]
pub enum Utils {
    Cn,
}

impl Utils {
    pub fn get_ui(&self) -> &'static UiComponent  {
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
