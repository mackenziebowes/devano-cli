use crate::library::client::utils::utils::UiComponent;

pub const LOG_IN_FORM_LIT: &str = r#"
import { Button } from "~/devano/atoms/buttons/Button";
import { TextInput } from "~/devano/atoms/inputs/TextInput";
import { PasswordInput } from "~/devano/atoms/inputs/PasswordInput";
import { AuthNav } from "~/devano/features/auth/AuthNav";
import { Card } from "~/devano/atoms/layout/Card";
import { Heading } from "~/devano/workshop/Heading";
import { SimpleSeparator } from "~/devano/atoms/decorators/Separator";
import { api } from "~/devano/api";
import { ErrorMessage } from "~/devano/workshop/ErrorMessage";
import { useAuth } from "./AuthState";

export function LogInForm() {
	const { state, schema } = useAuth();
	async function login() {
		let t_email = state.email.get();
		let t_password = state.password.get();

		let loginResponse = await api.auth.login({
			email: t_email,
			password: t_password,
		});
	}
	return (
		<Card class="max-w-[65ch]">
			<Heading as="h2">Log In</Heading>
			<div class="flex flex-col gap-[12px]">
				<TextInput
					label="Email"
					placeholder="Enter your email"
					get={state.email.get}
					set={state.email.set}
					validationSchema={schema.email}
					onValidationError={state.error.set}
				/>
				<PasswordInput
					get={state.password.get}
					set={state.password.set}
					label="Password"
					validationSchema={schema.password}
					onValidationError={state.error.set}
				/>
				<Button onClick={login}>Log In</Button>
				<ErrorMessage when={state.error.get()}>
					{state.error.get()}
				</ErrorMessage>
			</div>
			<SimpleSeparator
				direction={"horizontal"}
				label="OR"
			/>
			<AuthNav />
		</Card>
	);
}
"#;

pub const LOG_IN_FORM: UiComponent = UiComponent {
    name: "log-in-form",
    filename: "LogInForm.tsx",
    contents: LOG_IN_FORM_LIT,
    description: "Form for logging in",
    long_description: "Roll your own function to call backend for now :)",
    folder_path: "features/auth",
    npm_deps: &[],
};
