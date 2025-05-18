use crate::library::client::utils::utils::UiComponent;

#[derive(Debug, Clone, Copy)]
pub enum Feedbacks {
    ErrorMsg,
}

/// Retrieves the UI component associated with the feedback type.
///
/// # Returns
/// A reference to a static `UiComponent` that corresponds to the feedback type.
///
/// # Example
/// ```rust
/// let feedback = Feedback::ErrorMsg;
/// let ui_component = feedback.get_ui();
/// ```
impl Feedbacks {
    pub fn get_ui(&self) -> &'static UiComponent {
        match &self {
            Feedbacks::ErrorMsg => &ERROR_MSG,
        }
    }
}

pub const ERROR_MSG_LIT: &str = r#"
import { cn } from "~/devano/utils/cn";
import { JSX, splitProps, Show } from "solid-js";

interface ErrorMessageProps extends JSX.HTMLAttributes<HTMLDivElement> {
	when: boolean;
}

/**
 *
 * @props when Required - when to show the message
 * @props children What to show inside the message - use a fragment `<></>` for multiple children
 * @props class Extend/override default styles
 * @returns
 */
export function ErrorMessage(props: ErrorMessageProps) {
	const [l, rest] = splitProps(props, ["when", "class", "children"]);
	const errorCn = cn([
		"flex gap-[6px] px-[12px] py-[6px] bg-(--c-e-a) text-(--c-e-i)",
		l?.class,
	]);
	return (
		<Show when={l.when}>
			<div
				class={errorCn}
				{...rest}
			>
				{l.children}
			</div>
		</Show>
	);
}
"#;

pub const ERROR_MSG: UiComponent = UiComponent {
    name: "error-msg",
    filename: "ErrorMessage.tsx",
    contents: ERROR_MSG_LIT,
    description: "Feeback component for errors - uses the third palette color",
    long_description: "",
    folder_path: "atoms/feedback",
    npm_deps: &[],
};
