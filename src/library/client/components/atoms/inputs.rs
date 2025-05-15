use crate::library::client::utils::utils::{UiComponent};

#[derive(Debug, Clone, Copy)]
pub enum Inputs {
	Text,
	Password
}

impl Inputs {
	pub fn get_ui(&self) -> &'static UiComponent  {
		match &self {
			Inputs::Text => &TEXT_INPUT,
            Inputs::Password => &PASSWORD_INPUT,
		}
	}
}

/// LITERALS

pub const TEXT_INPUT_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface ExtendedTextInputProps
	extends JSX.InputHTMLAttributes<HTMLInputElement> {
	label: string;
}

export function TextInput(props: ExtendedTextInputProps) {
	const [l, rest] = splitProps(props, ["class", "label", "placeholder"]);

	let className = cn([
		"select-none bg-(--bg-a) hover:bg-(--bg-e) rounded-[6px] px-[6px] py-[6px] border-[2px] border-(--fg-e) hover:border(--fg-i) focus:outline-[1px] focus:-outline-offset-[2px] focus:outline-(--c-a-e)",
		l?.class,
	]);

	return (
		<div class="flex flex-col gap-[2px]">
			<label class="text-[14px] select-none ">{l.label}</label>
			<input
				placeholder={l?.placeholder || l.label}
				class={className}
				{...rest}
			/>
		</div>
	);
}
"#;

pub const TEXT_INPUT: UiComponent = UiComponent {
    name: "text-input",
    filename: "TextInput.tsx",
    contents: TEXT_INPUT_LIT,
    description: "Devano <input type='text'> implementation.",
	long_description: "",
	folder_path: "atoms/inputs",
	npm_deps: &[],
};


pub const PASSWORD_INPUT_LIT: &str = r#"
import { JSX, Match, splitProps, Switch } from "solid-js";
import { cn } from "~/devano/utils/cn";
import { createSignal } from "solid-js";
import IconButton from "~/devano/atoms/buttons/IconButton";
import EyeOpen from "~/devano/atoms/icons/EyeOpen";
import EyeClosed from "~/devano/atoms/icons/EyeClosed";

interface ExtendedTextInputProps
	extends JSX.InputHTMLAttributes<HTMLInputElement> {
	label: string;
}

export function PasswordInput(props: ExtendedTextInputProps) {
	const [vis, setVis] = createSignal(false);
	const [l, rest] = splitProps(props, ["class", "label", "placeholder"]);

	let className = cn([
		"select-none focus:outline-none focus:border-[transparent] flex-1",
		l?.class,
	]);

	return (
		<div class="flex flex-col gap-[2px]">
			<label class="text-[14px] select-none">{l.label}</label>
			<div class="has-[input:focus-within]:outline-[1px] has-[input:focus-within]:-outline-offset-[2px] has-[input:focus-within]:outline-(--c-a-e) flex bg-(--bg-a) hover:bg-(--bg-e) rounded-[6px] px-[6px] py-[6px] border-[2px] border-(--fg-e) hover:border(--fg-i)">
				<input
					type={vis() ? "text" : "password"}
					placeholder={l?.placeholder || l.label}
					class={className}
					{...rest}
				/>
				<Switch fallback={<></>}>
					<Match when={vis() == false}>
						<IconButton onClick={() => setVis(true)}>
							<EyeOpen class="w-[24px]" />
						</IconButton>
					</Match>
					<Match when={vis() == true}>
						<IconButton onClick={() => setVis(false)}>
							<EyeClosed class="w-[24px]" />
						</IconButton>
					</Match>
				</Switch>
			</div>
		</div>
	);
}
"#;

/// EXPORTS

pub const PASSWORD_INPUT: UiComponent = UiComponent {
	name: "password-input",
	filename: "PasswordInput.tsx",
	contents: PASSWORD_INPUT_LIT,
	description: "Stateful password implementation.",
	long_description: "",
	folder_path: "atoms/inputs",
	npm_deps: &[],
};