use crate::library::client::utils::utils::UiComponent;

#[derive(Debug, Clone, Copy)]
pub enum Decorators {
    Separators,
}

/// Retrieves the associated UI component for the `Decorators` instance.
///
/// # Returns
/// A reference to a static `UiComponent` that corresponds to the specific
/// variant of the `Decorators` enum. Currently, this method supports the
/// `Decorators::Separators` variant, returning a reference to the `SEPARATORS`
/// UI component.
impl Decorators {
    pub fn get_ui(&self) -> &'static UiComponent {
        match &self {
            Decorators::Separators => &SEPARATORS,
        }
    }
}

pub const SEPARATORS_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";
interface SeparatorProps extends JSX.HTMLAttributes<HTMLDivElement> {
	label?: string;
	direction?: "horizontal" | "vertical";
}

export function SimpleSeparator(props: SeparatorProps) {
	const [l, rest] = splitProps(props, [
		"label",
		"direction",
		"class",
		"aria-hidden",
	]);
	let dir = l?.direction ?? "horizontal";
	let containerCn = cn([
		"flex align-center items-center justify-center select-none",
		{
			"flex-col": dir == "vertical",
		},
		{
			"gap-[12px]": l?.label !== undefined,
		},
	]);
	let decoratorCn = cn([
		{
			"w-[2px] h-[32px]": l?.label == undefined && dir == "vertical",
			"w-[32px] h-[2px]": l?.label == undefined && dir == "horizontal",
			"w-[2px] h-[16px]": l?.label !== undefined && dir == "vertical",
			"w-[16px] h-[2px]": l?.label !== undefined && dir == "horizontal",
		},
		l.class?.includes("text-")
			? l.class
					.split(" ")
					.map((cls) =>
						cls.startsWith("text-") ? cls.replace("text-", "bg-") : cls
					)
					.join(" ")
			: "bg-(--fg-e)",
	]);

	return (
		<div
			class={containerCn}
			{...rest}
			aria-hidden
		>
			<div class={decoratorCn} />
			{l?.label}
			<div class={decoratorCn} />
		</div>
	);
}

interface SingleSimpleSeparatorProps
	extends JSX.HTMLAttributes<HTMLDivElement> {
	direction?: "horizontal" | "vertical";
}

export function SingleSimpleSeparator(props: SingleSimpleSeparatorProps) {
	const [l, rest] = splitProps(props, ["direction", "class"]);
	let dir = l?.direction ?? "horizontal";

	let containerCn = cn([
		"flex align-center items-center justify-center select-none",
		{
			"flex-col": dir == "vertical",
		},
	]);
	let decoratorCn = cn([
		{
			"w-[2px] h-[16px]": dir == "vertical",
			"w-[16px] h-[2px]": dir == "horizontal",
		},
		l.class?.includes("text-")
			? l.class
					.split(" ")
					.map((cls) =>
						cls.startsWith("text-") ? cls.replace("text-", "bg-") : cls
					)
					.join(" ")
			: "bg-(--fg-e)",
	]);

	return (
		<div
			class={containerCn}
			{...rest}
			aria-hidden
		>
			<div class={decoratorCn} />
		</div>
	);
}
"#;

pub const SEPARATORS: UiComponent = UiComponent {
    name: "separators",
    filename: "Separator.tsx",
    contents: SEPARATORS_LIT,
    description: "Separator decoration components",
    long_description: "",
    folder_path: "atoms/decorators",
    npm_deps: &[],
};
