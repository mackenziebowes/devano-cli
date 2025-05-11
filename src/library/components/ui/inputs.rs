use super::utils::{UiComponent};

/// LITERALS

pub const TEXT_INPUT_LITERAL: &str = r#"
import { JSX, splitProps } from "solid-js";
import twmerge from "tailwind-merge";

interface DevanoTextInputProps extends JSX.HTMLAttributes<HTMLInputElement> {}

export function TextInput(props: DevanoTextInputProps) {
	const [l, rest] = splitProps(props, ["class"]);
	return (
		<input
			class="bg-(--dc-bg-darker) py-(--ds-border-md) px-(--ds-gap-xs) rounded-(--ds-gap-xs) border-(length:--ds-border-sm) border-(--dc-fg) focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg)"
			{...rest}
		/>
	);
}
"#;


/// EXPORTS

pub const TEXT_INPUT: UiComponent = UiComponent {
    name: "text-input",
    filename: "TextInput.tsx",
    contents: TEXT_INPUT_LITERAL,
    required_npm: &["tailwind-merge","clsx"],
    description: "Devano <input type='text'> implementation.",
	long_description: "Uses tailwind-merge and clsx under the hood for conditional rendering and custom class props."
};