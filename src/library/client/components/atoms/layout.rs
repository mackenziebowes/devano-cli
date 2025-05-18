use crate::library::client::utils::utils::UiComponent;

#[derive(Debug, Clone, Copy)]
pub enum Layouts {
    Card,
    Page,
    PageInner,
    Stack,
}

/// Returns a reference to the corresponding `UiComponent` for the given layout variant.
///
/// # Returns
/// A static reference to the `UiComponent` associated with the current `Layout`.
///
/// # Examples
/// ```
/// let layout = Layout::Card;
/// let ui_component = layout.get_ui();
/// ```
impl Layouts {
    pub fn get_ui(&self) -> &'static UiComponent {
        match &self {
            Layouts::Card => &CARD,
            Layouts::Page => &PAGE,
            Layouts::PageInner => &PAGE_INNER,
            Layouts::Stack => &STACK,
        }
    }
}

pub const PAGE_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";

export function Page(props: JSX.HTMLAttributes<HTMLElement>) {
	const [l, rest] = splitProps(props, ["class"]);
	return (
		<main
			class="flex flex-col w-full min-h-[100vh] gap-[48px] items-center bg-(--bg-i) text-(--fg-e)"
			{...rest}
		/>
	);
}
"#;

pub const PAGE: UiComponent = UiComponent {
    name: "page",
    filename: "Page.tsx",
    contents: PAGE_LIT,
    description: "Devano <main> implementation.",
    long_description: "",
    folder_path: "atoms/layout",
    npm_deps: &[],
};

// todo: figure out nav location + config for extension
pub const PAGE_INNER_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";
// import { TopNav } from "./TopNav";
// import { FooterNav } from "./FooterNav";

interface PageInnerProps extends JSX.HTMLAttributes<HTMLDivElement> {}

export default function PageInner(props: PageInnerProps) {
	const [l, rest] = splitProps(props, ["class"]);

	const innerCn = cn([
		"flex flex-col h-screen w-screen items-center justify-between",
		l?.class,
	]);

	return (
		<div
			class={innerCn}
			{...rest}
		>
			<div class="flex-none w-full px-[24px] py-[12px]">
				{/* <TopNav /> */}
			</div>
			<div class="flex-1 overflow-y-scroll pb-[24px]">{props.children}</div>
			<div class="flex items-center justify-center flex-none w-full py-[12px]">
				{/* <FooterNav /> */}
			</div>
		</div>
	);
}
"#;

pub const PAGE_INNER: UiComponent = UiComponent {
    name: "page-inner",
    filename: "PageInner.tsx",
    contents: PAGE_INNER_LIT,
    description: "Composes navigation... WIP",
    long_description: "",
    folder_path: "atoms/layout",
    npm_deps: &[],
};

pub const CARD_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface CardProps extends JSX.HTMLAttributes<HTMLDivElement> {}

export function Card(props: CardProps) {
	let [l, rest] = splitProps(props, ["class", "children"]);

	let cardCN = cn([
		"flex flex-col w-[35ch] border-[2px] border-(--gh-e) px-[24px] py-[12px] pb-[24px] rounded-[12px] gap-[24px]",
		l?.class,
	]);

	return (
		<div
			class={cardCN}
			{...rest}
		>
			{l.children}
		</div>
	);
}
"#;

pub const CARD: UiComponent = UiComponent {
    name: "card",
    filename: "Card.tsx",
    contents: CARD_LIT,
    description: "Devano card implementation",
    long_description: "Append a class to over-ride default styles",
    folder_path: "atoms/layout",
    npm_deps: &[],
};

pub const STACK_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface StackProps extends JSX.HTMLAttributes<HTMLDivElement> {
	direction?: "row" | "col";
}

/**
 *
 * 	@prop class standard classnames, use to extend or overwrite defaults.
 *	@prop direction "row" | "col" -> makes a row or column
 */
export default function Stack(props: StackProps) {
	const [l, rest] = splitProps(props, ["children", "class", "direction"]);

	let className = cn([
		"flex gap-[12px] items-center",
		{
			"flex-col gap-[6px]": l?.direction === "col",
		},
		l?.class,
	]);

	return (
		<div
			class={className}
			{...rest}
		>
			{l.children}
		</div>
	);
}
"#;

pub const STACK: UiComponent = UiComponent {
    name: "stack",
    filename: "Stack.tsx",
    contents: STACK_LIT,
    description: "Devano flex-row/flex-column implementation",
    long_description: "Use the Direction prop for fast row/columns. Row by default.",
    folder_path: "atoms/layout",
    npm_deps: &[],
};
