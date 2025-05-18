use crate::library::client::utils::utils::UiComponent;

pub const AUTH_INNER_LIT: &str = r#"
import { LogInForm } from "~/devano/features/auth/LogInForm";
import { RegisterForm } from "~/devano/features/auth/RegisterForm";
import { PasswordResetRequest } from "./PasswordResetRequest";
import { PasswordResetChallenge } from "./PasswordResetChallenge";
import { Switch, Match } from "solid-js";
import { useAuth, ViewState } from "~/devano/features/auth/AuthState";

export default function AuthInner() {
	const { view } = useAuth();
	return (
		<div>
			<Switch>
				<Match when={view() === ViewState.LogIn}>
					<LogInForm />
				</Match>
				<Match when={view() === ViewState.Register}>
					<RegisterForm />
				</Match>
				<Match when={view() === ViewState.ResetPasswordRequest}>
					<PasswordResetRequest />
				</Match>
				<Match when={view() === ViewState.ResetPasswordChallenge}>
					<PasswordResetChallenge />
				</Match>
			</Switch>
		</div>
	);
}
"#;

pub const AUTH_INNER: UiComponent = UiComponent {
    name: "auth-inner",
    filename: "AuthInner.tsx",
    contents: AUTH_INNER_LIT,
    description: "The view controller for Auth.",
    long_description: "Handles switches between view states.",
    folder_path: "features/auth",
    npm_deps: &[],
};
