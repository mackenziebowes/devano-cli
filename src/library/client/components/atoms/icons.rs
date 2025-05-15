use crate::library::client::utils::utils::{UiComponent};

#[derive(Debug, Clone, Copy)]
pub enum Icons {
   EyeClosed,
   EyeOpen,
}


impl Icons {
    pub fn get_ui(&self) -> &'static UiComponent  {
		match &self {
			Icons::EyeClosed => &EYE_CLOSED,
            Icons::EyeOpen => &EYE_OPEN,
		}
	}
}

/// LITERALS

pub const EYE_CLOSED_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

export default function SVGArt(props: JSX.SvgSVGAttributes<SVGSVGElement>) {
	const [l, rest] = splitProps(props, ["width", "height", "class"]);
	return (
		<svg
			width={l.width ?? "100"}
			height={l.height ?? "60"}
			viewBox="0 0 100 60"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
			class={cn(["fill-current h-auto", l?.class])} // fill-current consumes the parent's "text color" value, h-auto makes it easy to proportionally resize the art
			{...rest}
		>
			<path d="M90 13.5C91.3807 13.5 92.5 14.6193 92.5 16C92.5 23.5926 89.7805 31.7988 82.9424 38.0898C76.1075 44.3779 65.4361 48.5 50 48.5C34.5639 48.5 23.8925 44.3779 17.0576 38.0898C10.2195 31.7988 7.5 23.5926 7.5 16C7.5 14.6193 8.61929 13.5 10 13.5C11.3807 13.5 12.5 14.6193 12.5 16C12.5 22.4073 14.7806 29.2012 20.4424 34.4102C26.1075 39.622 35.4362 43.5 50 43.5C64.5638 43.5 73.8925 39.622 79.5576 34.4102C85.2194 29.2012 87.5 22.4073 87.5 16C87.5 14.6193 88.6193 13.5 90 13.5Z" />
		</svg>
	);
}
"#;

pub const EYE_OPEN_LIT: &str = r#"
import { JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

export default function SVGArt(props: JSX.SvgSVGAttributes<SVGSVGElement>) {
	const [l, rest] = splitProps(props, ["width", "height", "class"]);
	return (
		<svg
			width={l.width ?? "100"}
			height={l.height ?? "60"}
			viewBox="0 0 100 60"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
			class={cn(["fill-current h-auto", l?.class])} // fill-current consumes the parent's "text color" value, h-auto makes it easy to proportionally resize the art
			{...rest}
		>
			<path d="M50.0117 24.5C56.9153 24.5 62.5117 25.0964 62.5117 34C62.5117 40.9036 56.9153 46.5 50.0117 46.5C43.1082 46.5 37.5117 40.9036 37.5117 34C37.5117 25.0964 43.1082 24.5 50.0117 24.5Z" />
			<path d="M50.0117 7.5C81.3924 7.5 92.5117 28.6193 92.5117 40C92.5117 41.3807 91.3924 42.5 90.0117 42.5C88.631 42.5 87.5117 41.3807 87.5117 40C87.5117 31.3807 78.631 12.5 50.0117 12.5C21.3924 12.5 12.5117 31.3807 12.5117 40C12.5117 41.3807 11.3924 42.5 10.0117 42.5C8.63101 42.5 7.51172 41.3807 7.51172 40C7.51172 28.6193 18.631 7.5 50.0117 7.5Z" />
		</svg>
	);
}
"#;

pub const EYE_CLOSED: UiComponent = UiComponent {
    name: "eye-closed-icon",
    filename: "EyeClosed.tsx",
    contents: EYE_CLOSED_LIT,
    description: "Eye closed icon, included for visibility controls",
	long_description: "",
	folder_path: "atoms/icons",
	npm_deps: &[],
};

pub const EYE_OPEN: UiComponent = UiComponent {
    name: "eye-open-icon",
    filename: "EyeOpen.tsx",
    contents: EYE_OPEN_LIT,
    description: "Eye open icon, included for visibility controls",
	long_description: "",
	folder_path: "atoms/icons",
	npm_deps: &[],
};