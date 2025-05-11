use super::utils::{UiComponent};

/// LITERALS

pub const BUTTON_COMPONENT_LITERAL: &str = r#"
import { JSX, splitProps } from "solid-js";

interface DevanoButtonProps extends JSX.HTMLAttributes<HTMLButtonElement> {
	outline?: boolean;
	color?: "default" | "primary" | "secondary" | "destructive";
	label?: string;
}
export function Button(props: DevanoButtonProps) {
	const [l, rest] = splitProps(props, ["outline", "color", "label", "class"]);
	const color = l.color;
	if (color === "destructive") {
		if (l?.outline === true) {
			return (
				<button
					class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-dbg) text-(--dc-dbg)  hover:text-(--dc-dbg-brighter) hover:border-(--dc-dbg-brighter) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg) ${l?.class}`}
					{...rest}
				>
					{l?.label ?? "Outline Button Primary"}
				</button>
			);
		}
		return (
			<button
				class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-dbg) text-(--dc-pfg) bg-(--dc-dbg) hover:bg-(--dc-dbg-brighter) hover:border-(--dc-dbg-brighter) hover:text-(--dc-pfg-darker) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg) ${l?.class}`}
				{...rest}
			>
				{l?.label ?? "Outline Button Primary"}
			</button>
		);
	}
	if (color === "secondary") {
		if (l?.outline === true) {
			return (
				<button
					class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-sbg) text-(--dc-sbg)  hover:text-(--dc-sbg-brighter) hover:border-(--dc-sbg-brighter) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg) ${l?.class}`}
					{...rest}
				>
					{l?.label ?? "Outline Button Primary"}
				</button>
			);
		}
		return (
			<button
				class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-sbg) text-(--dc-pfg) bg-(--dc-sbg) hover:bg-(--dc-sbg-brighter) hover:border-(--dc-sbg-brighter) hover:text-(--dc-pfg-darker) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg) ${l?.class}`}
				{...rest}
			>
				{l?.label ?? "Outline Button Primary"}
			</button>
		);
	}

	if (color === "primary") {
		if (l?.outline === true) {
			return (
				<button
					class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-pbg) text-(--dc-pbg)  hover:text-(--dc-pbg-brighter) hover:border-(--dc-pbg-brighter) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-sbg) ${l?.class}`}
					{...rest}
				>
					{l?.label ?? "Outline Button Primary"}
				</button>
			);
		}
		return (
			<button
				class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-pbg) text-(--dc-pfg) bg-(--dc-pbg) hover:bg-(--dc-pbg-brighter) hover:border-(--dc-pbg-brighter) hover:text-(--dc-pfg-darker) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-sbg) ${l?.class}`}
				{...rest}
			>
				{l?.label ?? "Outline Button Primary"}
			</button>
		);
	}
	if (l?.outline === true) {
		return (
			<button
				class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-fg) text-(--dc-fg) hover:text-(--dc-fg-brighter) hover:border-(--dc-fg-brighter) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg) ${l?.class}`}
				{...rest}
			>
				{l?.label ?? "Outline Button Primary"}
			</button>
		);
	}
	return (
		<button
			class={`px-4 py-1 font-semibold rounded-md border-(length:--ds-border-sm) border-(--dc-fg) text-(--dc-bg) bg-(--dc-fg) hover:bg-(--dc-fg-brighter) hover:border-(--dc-fg-brighter) hover:text-(--dc-bg-darker) hover:cursor-pointer focus:outline-(length:--ds-border-xs) focus:outline-(--dc-pbg) ${l?.class}`}
			{...rest}
		>
			{l?.label ?? "Outline Button Primary"}
		</button>
	);
}

"#;



/// EXPORTS

pub const BUTTON: UiComponent = UiComponent {
    name: "button",
    filename: "Button.tsx",
    contents: BUTTON_COMPONENT_LITERAL,
    required_npm: &["@devano/ui"],
    description: "Devano <button> implementation.",
	long_description: "Uses tailwind-merge and clsx under the hood for conditional rendering and custom class props."
};