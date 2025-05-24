use crate::library::client::utils::utils::UiComponent;

#[derive(Debug, Clone, Copy)]
pub enum Buttons {
    Anchor,
    AnchorButton,
    Button,
    ButtonAnchor,
    IconButton,
}

/// Provides a method to retrieve the associated `UiComponent` for a given button type.
///
/// # Method
/// - `get_ui`: Returns a reference to the static `UiComponent` corresponding to the button variant.
///
/// # Variants
/// - `Buttons::Anchor`: Returns the `ANCHOR` component.
/// - `Buttons::AnchorButton`: Returns the `ANCHOR_BUTTON` component.
/// - `Buttons::Button`: Returns the `BUTTON` component.
/// - `Buttons::ButtonAnchor`: Returns the `BUTTON_ANCHOR` component.
/// - `Buttons::IconButton`: Returns the `ICON_BUTTON` component.
///
/// # Returns
/// A reference to a static `UiComponent` that represents the UI element for the button variant.
impl Buttons {
    pub fn get_ui(&self) -> &'static UiComponent {
        match &self {
            Buttons::Anchor => &ANCHOR,
            Buttons::AnchorButton => &ANCHOR_BUTTON,
            Buttons::Button => &BUTTON,
            Buttons::ButtonAnchor => &BUTTON_ANCHOR,
            Buttons::IconButton => &ICON_BUTTON,
        }
    }
}

pub const ANCHOR_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { A } from "@solidjs/router";
import { cn } from "~/devano/utils/cn";

interface AnchorProps extends JSX.AnchorHTMLAttributes<HTMLAnchorElement> {
	color?: "default" | "ara" | "ene" | "izi";
	href: string;
	end?: boolean;
}

export function Anchor(props: AnchorProps) {
	const [l, rest] = splitProps(props, [
		"color",
		"class",
		"children",
		"href",
		"end",
	]);

	const color = l?.color ?? "default";
	const end = l?.end ?? true;

	let inactiveCn = cn([
		"select-none font-semibold hover:cursor-pointer focus:outline-none", 
		{
			"text-(--fg-e) hover:text-(--fg-i) focus:text-(--c-a-e)":
				color == "default",
			"text-(--c-a-e) hover:text-(--c-a-i) focus:text-(--fg-i)": color == "ara", 
			"text-(--c-e-e) hover:text-(--c-e-i) focus:text-(--fg-i)": color == "ene",
			"text-(--c-i-e) hover:text-(--c-i-i) focus:text-(--fg-i)": color == "izi",
		},
		l?.class,
	]);

	let activeCn = cn([
		"select-none font-semibold hover:cursor-pointer focus:outline-none",
		{
			"text-(--fg-a) hover:text-(--fg-e) focus:text-(--c-a-i)":
				color == "default",
			"text-(--c-a-a) hover:text-(--c-a-e) focus:text-(--fg-i)": color == "ara",
			"text-(--c-e-a) hover:text-(--c-e-e) focus:text-(--fg-i)": color == "ene",
			"text-(--c-i-a) hover:text-(--c-i-e) focus:text-(--fg-i)": color == "izi",
		},
		l?.class,
	]);

	return (
		<A
			href={l.href}
			inactiveClass={inactiveCn}
			activeClass={activeCn}
			{...rest}
			end={end}
		>
			{l?.children ?? ""}
		</A>
	);
}
"#;

pub const ANCHOR: UiComponent = UiComponent {
    name: "anchor",
    filename: "Anchor.tsx",
    contents: ANCHOR_LIT,
    description: "Devano <a> implementation.",
    long_description: "",
    folder_path: "atoms/buttons",
    npm_deps: &[],
};

pub const ANCHOR_BUTTON_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface AnchorProps extends JSX.ButtonHTMLAttributes<HTMLButtonElement> {
	color?: "default" | "ara" | "ene" | "izi";
}

export function AnchorButton(props: AnchorProps) {
	const [l, rest] = splitProps(props, ["color", "class", "children"]);

	const color = l.color ?? "default";

	let className = cn([
		"select-none font-semibold hover:cursor-pointer focus:outline-none",
		{
			"text-(--fg-e) hover:text-(--fg-i) focus:text-(--c-a-e)":
				color == "default",
			"text-(--c-a-e) hover:text-(--c-a-i) focus:text-(--fg-i)": color == "ara",
			"text-(--c-e-e) hover:text-(--c-e-i) focus:text-(--fg-i)": color == "ene",
			"text-(--c-i-e) hover:text-(--c-i-i) focus:text-(--fg-i)": color == "izi",
		},
		l?.class,
	]);

	return (
		<button
			class={className}
			{...rest}
		>
			{l?.children ?? ""}
		</button>
	);
}
"#;

pub const ANCHOR_BUTTON: UiComponent = UiComponent {
    name: "anchor",
    filename: "AnchorButton.tsx",
    contents: ANCHOR_BUTTON_LIT,
    description: "Devano <a> implementation.",
    long_description: "",
    folder_path: "atoms/buttons",
    npm_deps: &[],
};

pub const BUTTON_ANCHOR_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface ButtonProps extends JSX.HTMLAttributes<HTMLButtonElement> {
	outline?: boolean;
	color?: "default" | "ara" | "ene" | "izi";
	label?: string;
}
export function Button(props: ButtonProps) {
	const [l, rest] = splitProps(props, [
		"outline",
		"color",
		"label",
		"class",
		"children",
	]);

	const color = l.color ?? "default";

	let className = cn([
		"select-none px-4 py-1 font-semibold rounded-md border-[2px] hover:cursor-pointer focus:outline-[1px] focus:outline-(--c-a-e)",
		{
			"border-(--fg-e) text-(--fg-e) hover:text-(--fg-i) hover:border-(--fg-i)":
				color == "default" && l?.outline == true,
			"border-(--fg-i) bg-(--fg-i) text-(--bg-e) hover:bg-(--fg-o) hover:text-(--bg-i) hover:border-(--fg-o)":
				color == "default" && l?.outline == undefined,
			"border-(--c-a-e) text-(--c-a-e) hover:text-(--c-a-i) hover:border-(--c-a-i)":
				color == "ara" && l?.outline == true,
			"border-(--c-a-e) text-(--bg-i) bg-(--c-a-e) hover:bg-(--c-a-i) hover:text-(--bg-o) hover:border-(--c-a-i)":
				color == "ara" && l?.outline == undefined,
			"border-(--c-e-e) text-(--c-e-e) hover:text-(--c-e-i) hover:border-(c-e-i)":
				color == "ene" && l?.outline == true,
			"border-(--c-e-e) text-(--bg-i) bg-(--c-e-e) hover:bg-(--c-e-i) hover:text-(--bg-o) hover:border-(--c-e-i)":
				color == "ene" && l?.outline == undefined,
			"border-(--c-i-e) text-(--c-i-e) hover:text-(--c-i-i) hover:border-(c-i-i)":
				color == "izi" && l?.outline == true,
			"border-(--c-i-e) text-(--bg-i) bg-(--c-i-e) hover:bg-(--c-i-i) hover:text-(--bg-o) hover:border-(--c-i-i)":
				color == "izi" && l?.outline == undefined,
		},
		l?.class,
	]);

	return (
		<a
			class={className}
			{...rest}
		>
			{l?.label ?? l?.children ?? ""}
		</a>
	);
}
"#;

pub const BUTTON_ANCHOR: UiComponent = UiComponent {
    name: "anchor-button",
    filename: "LinkButton.tsx",
    contents: BUTTON_ANCHOR_LIT,
    description: "Devano <a> implementation styled like a button.",
    long_description: "",
    folder_path: "atoms/buttons",
    npm_deps: &[],
};

pub const BUTTON_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface ButtonProps extends JSX.ButtonHTMLAttributes<HTMLButtonElement> {
	outline?: boolean;
	color?: "default" | "ara" | "ene" | "izi";
	label?: string;
}
export function Button(props: ButtonProps) {
	const [l, rest] = splitProps(props, [
		"outline",
		"color",
		"label",
		"class",
		"children",
	]);

	const color = l.color ?? "default";

	let className = cn([
		"select-none px-4 py-1 font-semibold rounded-md border-[2px] hover:cursor-pointer focus:outline-[1px] focus:outline-(--c-a-e)",
		{
			"border-(--fg-e) text-(--fg-e) hover:text-(--fg-i) hover:border-(--fg-i)":
				color == "default" && l?.outline == true,
			"border-(--fg-i) bg-(--fg-i) text-(--bg-e) hover:bg-(--fg-o) hover:text-(--bg-i) hover:border-(--fg-o)":
				color == "default" && l?.outline == undefined,
			"border-(--c-a-e) text-(--c-a-e) hover:text-(--c-a-i) hover:border-(--c-a-i)":
				color == "ara" && l?.outline == true,
			"border-(--c-a-e) text-(--bg-i) bg-(--c-a-e) hover:bg-(--c-a-i) hover:text-(--bg-o) hover:border-(--c-a-i)":
				color == "ara" && l?.outline == undefined,
			"border-(--c-e-e) text-(--c-e-e) hover:text-(--c-e-i) hover:border-(c-e-i)":
				color == "ene" && l?.outline == true,
			"border-(--c-e-e) text-(--bg-i) bg-(--c-e-e) hover:bg-(--c-e-i) hover:text-(--bg-o) hover:border-(--c-e-i)":
				color == "ene" && l?.outline == undefined,
			"border-(--c-i-e) text-(--c-i-e) hover:text-(--c-i-i) hover:border-(c-i-i)":
				color == "izi" && l?.outline == true,
			"border-(--c-i-e) text-(--bg-i) bg-(--c-i-e) hover:bg-(--c-i-i) hover:text-(--bg-o) hover:border-(--c-i-i)":
				color == "izi" && l?.outline == undefined,
		},
		l?.class,
	]);

	return (
		<button
			class={className}
			{...rest}
		>
			{l?.label ?? l?.children ?? ""}
		</button>
	);
}
"#;

pub const BUTTON: UiComponent = UiComponent {
    name: "button",
    filename: "Button.tsx",
    contents: BUTTON_LIT,
    description: "Devano <button> implementation.",
    long_description: "",
    folder_path: "atoms/buttons",
    npm_deps: &[],
};

pub const ICON_BUTTON_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface IconButtonProps extends JSX.HTMLAttributes<HTMLButtonElement> {
	outline?: boolean;
	color?: "default" | "ara" | "ene" | "izi";
}

export default function IconButton(props: IconButtonProps) {
	const [l, rest] = splitProps(props, [
		"outline",
		"color",
		"class",
		"children",
	]);

	const color = l.color ?? "default";

	let className = cn([
		"select-none px-1 py-1 font-semibold rounded-md border-[2px] hover:cursor-pointer focus:outline-[1px] focus:outline-(--c-a-e)",
		{
			"border-(--fg-e) text-(--fg-e) hover:text-(--fg-i) hover:border-(--fg-i)":
				color == "default" && l?.outline == true,
			"border-(--fg-i) bg-(--fg-i) text-(--bg-e) hover:text-(--bg-i) hover:bg-(--fg-o) hover:border-(--fg-o)":
				color == "default" && l?.outline == undefined,
			"border-(--c-a-e) text-(--c-a-e) hover:text-(--c-a-i) hover:border-(--c-a-i)":
				color == "ara" && l?.outline == true,
			"border-(--c-a-e) text-(--bg-i) bg-(--c-a-e) hover:bg-(--c-a-i) hover:text-(--bg-o) hover:border-(--c-a-i)":
				color == "ara" && l?.outline == undefined,
			"border-(--c-e-e) text-(--c-e-e) hover:text-(--c-e-i) hover:border-(c-e-i)":
				color == "ene" && l?.outline == true,
			"border-(--c-e-e) text-(--bg-i) bg-(--c-e-e) hover:bg-(--c-e-i) hover:text-(--bg-o) hover:border-(--c-e-i)":
				color == "ene" && l?.outline == undefined,
			"border-(--c-i-e) text-(--c-i-e) hover:text-(--c-i-i) hover:border-(c-i-i)":
				color == "izi" && l?.outline == true,
			"border-(--c-i-e) text-(--bg-i) bg-(--c-i-e) hover:bg-(--c-i-i) hover:text-(--bg-o) hover:border-(--c-i-i)":
				color == "izi" && l?.outline == undefined,
		},
		l?.class,
	]);

	return (
		<button
			class={className}
			{...rest}
		>
			{l.children ?? ""}
		</button>
	);
}
"#;

pub const ICON_BUTTON: UiComponent = UiComponent {
    name: "button",
    filename: "IconButton.tsx",
    contents: ICON_BUTTON_LIT,
    description: "Devano 'icon-button' implementation.",
    long_description: "Put an svg inside to give it standard Devano options and make it clicky.",
    folder_path: "atoms/buttons",
    npm_deps: &[],
};
