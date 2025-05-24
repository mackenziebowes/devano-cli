use crate::library::client::utils::utils::UiComponent;

#[derive(Debug, Clone, Copy)]
pub enum Inputs {
    Text,
    Password,
	Otp,
}

impl Inputs {
    pub fn get_ui(&self) -> &'static UiComponent {
        match &self {
            Inputs::Text => &TEXT_INPUT,
            Inputs::Password => &PASSWORD_INPUT,
			Inputs::Otp => &OTP,
        }
    }
}

/// LITERALS

pub const TEXT_INPUT_LIT: &str = r#"
import { JSX, splitProps, createSignal } from "solid-js";
import { cn } from "~/devano/utils/cn";
import type { Accessor, Setter } from "solid-js";
import { ZodType, ZodError } from "zod";

interface ExtendedTextInputProps
	extends JSX.InputHTMLAttributes<HTMLInputElement> {
	label: string;
	get: Accessor<string>;
	set: Setter<string>;
	validationSchema?: ZodType<any>;
	onValidationError?: (error: string) => void;
}

export function TextInput(props: ExtendedTextInputProps) {
	const [l, most] = splitProps(props, [
		"class",
		"label",
		"placeholder",
		"get",
		"set",
		"validationSchema",
		"onValidationError",
	]);
	const [unused, rest] = splitProps(most, [
		"value",
		"on:change",
		"onChange",
		"onchange",
		"on:input",
		"onInput",
		"oninput",
		"on:blur",
		"onBlur",
		"onblur",
	]);

	const [error, set_error] = createSignal<string | null>(null);

	let className = cn([
		"select-none bg-(--bg-a) hover:bg-(--bg-e) rounded-[6px] px-[6px] py-[6px] border-[2px] border-(--fg-e) hover:border(--fg-i) focus:outline-[1px] focus:-outline-offset-[2px] focus:outline-(--c-a-e)",
		l?.class,
		error() ? "border-(--c-e-e) hover:border-(--c-e-i)" : "",
	]);

	const handleBlur = () => {
		if (l.validationSchema) {
			try {
				l.validationSchema.parse(l.get());
				set_error(null);
			} catch (err) {
				if (err instanceof ZodError) {
					const errorMessage = err.errors[0]?.message || "Invalid Input";
					set_error(errorMessage);
					if (l.onValidationError) {
						l.onValidationError(errorMessage);
					}
				}
			}
		}
	};

	return (
		<div class="flex flex-col gap-[2px]">
			<label class="text-[14px] select-none ">{l.label}</label>
			<input
				placeholder={l?.placeholder || l.label}
				class={className}
				value={l.get()}
				onInput={(evt) => l.set(evt.currentTarget.value)}
				onBlur={handleBlur}
				{...rest}
			/>
			{error() && <span class="text-(--c-e-e) text-[12px]">{error()}</span>}
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
    npm_deps: &["zod"],
};

pub const PASSWORD_INPUT_LIT: &str = r#"
import { JSX, Match, splitProps, Switch, createSignal } from "solid-js";
import { cn } from "~/devano/utils/cn";
import IconButton from "~/devano/atoms/buttons/IconButton";
import EyeOpen from "~/devano/atoms/icons/EyeOpen";
import EyeClosed from "~/devano/atoms/icons/EyeClosed";
import type { Accessor, Setter } from "solid-js";
import { ZodType, ZodError } from "zod";

interface ExtendedPasswordInputProps
	extends JSX.InputHTMLAttributes<HTMLInputElement> {
	label: string;
	get: Accessor<string>;
	set: Setter<string>;
	validationSchema?: ZodType<any>;
	onValidationError?: (error: string) => void;
}

export function PasswordInput(props: ExtendedPasswordInputProps) {
	const [vis, setVis] = createSignal(false);
	const [l, most] = splitProps(props, [
		"class",
		"label",
		"placeholder",
		"get",
		"set",
		"validationSchema",
		"onValidationError",
	]);
	const [unused, rest] = splitProps(most, [
		"value",
		"on:change",
		"onChange",
		"onchange",
		"on:input",
		"onInput",
		"oninput",
		"on:blur",
		"onBlur",
		"onblur",
	]);

	const [error, set_error] = createSignal<string | null>(null);

	let className = cn([
		"select-none focus:outline-none focus:border-[transparent] flex-1",
		l?.class,
		error() ? "border-(--c-e-e) hover:border-(--c-e-i)" : "",
	]);

	const handleBlur = () => {
		if (l.validationSchema) {
			try {
				l.validationSchema.parse(l.get());
				set_error(null);
			} catch (err) {
				if (err instanceof ZodError) {
					const errorMessage = err.errors[0]?.message || "Invalid Input";
					set_error(errorMessage);
					if (l.onValidationError) {
						l.onValidationError(errorMessage);
					}
				}
			}
		}
	};

	return (
		<div class="flex flex-col gap-[2px]">
			<label class="text-[14px] select-none">{l.label}</label>
			<div class="has-[input:focus-within]:outline-[1px] has-[input:focus-within]:-outline-offset-[2px] has-[input:focus-within]:outline-(--c-a-e) flex bg-(--bg-a) hover:bg-(--bg-e) rounded-[6px] px-[6px] py-[6px] border-[2px] border-(--fg-e) hover:border(--fg-i)">
				<input
					type={vis() ? "text" : "password"}
					placeholder={l?.placeholder || l.label}
					class={className}
					value={l.get()}
					onInput={(evt) => l.set(evt.currentTarget.value)}
					onBlur={handleBlur}
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
			{error() && <span class="text-(--c-e-e) text-[12px]">{error()}</span>}
		</div>
	);
}
"#;

pub const PASSWORD_INPUT: UiComponent = UiComponent {
    name: "password-input",
    filename: "PasswordInput.tsx",
    contents: PASSWORD_INPUT_LIT,
    description: "Stateful password implementation.",
    long_description: "",
    folder_path: "atoms/inputs",
    npm_deps: &["zod"],
};

pub const OTP_LIT: &str = r#"
import { createSignal, For, JSX, splitProps } from "solid-js";
import { cn } from "~/devano/utils/cn";

interface OTPInputProps extends JSX.InputHTMLAttributes<HTMLInputElement> {
	label: string;
	length: number;
	onComplete?: (value: string) => void;
	type?: "single" | "bubble";
}

export function OTPInput(props: OTPInputProps) {
	const [l, rest] = splitProps(props, [
		"class",
		"label",
		"length",
		"onComplete",
		"type",
	]);

	let type = l?.type ?? "single";

	const [values, setValues] = createSignal(Array(l.length).fill(""));

	let inputs: HTMLInputElement[] = [];

	const handleInput = (index: number, event: Event) => {
		const target = event.target as HTMLInputElement;
		const newValue = target.value.slice(-1); // Only take the last character
		const updatedValues = [...values()];
		updatedValues[index] = newValue;
		setValues(updatedValues);

		// Move to the next input if available
		if (newValue && index < l.length - 1) {
			inputs[index + 1]?.focus();
		}

		// Call onComplete if all fields are filled
		if (updatedValues.every((v) => v !== "") && l.onComplete) {
			l.onComplete(updatedValues.join(""));
		}
	};

	const handleKeyDown = (index: number, event: KeyboardEvent) => {
		if (event.key === "Backspace" && !values()[index] && index > 0) {
			inputs[index - 1]?.focus();
		}
	};

	let containerCn = cn([
		"flex justify-center items-center gap-[4px] text-2xl",
		{
			"bg-(--bg-a) hover:bg-(--bg-e) rounded-[6px] px-[6px] py-[6px] border-[2px] border-(--fg-e) hover:border(--fg-i) has-[input:focus-within]:outline-[1px] has-[input:focus-within]:-outline-offset-[2px] has-[input:focus-within]:outline-(--c-a-e)":
				type === "single",
		},
		{
			"justify-between": type === "bubble",
		},
	]);
	let wrapperCn = cn([
		"select-none aspect-square flex justify-center items-center",
		{
			"bg-(--bg-a) hover:bg-(--bg-e) p-3 rounded-[6px] border-[2px] border-(--fg-e) hover:border(--fg-i) has-[input:focus-within]:outline-[1px] has-[input:focus-within]:-outline-offset-[2px] has-[input:focus-within]:outline-(--c-a-e)":
				type === "bubble",
		},
	]);
	let inputCn = cn([
		"select-none w-[2.5ch] aspect-square focus:outline-none text-center",
		l?.class,
	]);

	return (
		<div class="flex flex-col gap-[2px]">
			<label class="text-[14px] select-none">{l.label}</label>
			<div
				class={containerCn}
				onClick={() => inputs[0]?.focus()}
			>
				<For each={Array(l.length)}>
					{(_, index) => (
						<div class={wrapperCn}>
							<input
								ref={(el) => (inputs[index()] = el)}
								type="text"
								maxLength={1}
								value={values()[index()]}
								class={inputCn}
								onInput={(e) => handleInput(index(), e)}
								onKeyDown={(e) => handleKeyDown(index(), e)}
								onFocus={(e) => (e.target as HTMLInputElement).select()}
								{...rest}
							/>
						</div>
					)}
				</For>
			</div>
		</div>
	);
}
"#;

pub const OTP: UiComponent = UiComponent {
	name: "otp-input",
    filename: "OTPInput.tsx",
    contents: OTP_LIT,
    description: "Stateful OTP Implementation.",
    long_description: "",
    folder_path: "atoms/inputs",
    npm_deps: &[],
};