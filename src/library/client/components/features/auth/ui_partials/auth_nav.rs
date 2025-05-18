use crate::library::client::utils::utils::UiComponent;

pub const AUTH_NAV_LIT: &str = r#"
import { useAuth, ViewState } from "~/devano/features/auth/AuthState";
import { SingleSimpleSeparator } from "~/devano/atoms/decorators/Separator";
import { AnchorButton } from "~/devano/atoms/buttons/AnchorButton";

export function AuthNav() {
	let { view, setViewTo } = useAuth();

	return (
		<div class="flex align-center w-full gap-[6px] justify-center">
			{[
				view() !== ViewState.LogIn && (
					<AnchorButton onClick={setViewTo.login}>Log In</AnchorButton>
				),
				view() !== ViewState.Register && (
					<AnchorButton onClick={setViewTo.register}>Register</AnchorButton>
				),
				view() !== ViewState.ResetPasswordRequest && (
					<AnchorButton onClick={setViewTo.resetpasswordreq}>
						Reset Password
					</AnchorButton>
				),
			]
				.filter(Boolean)
				.map((item, index, array) => (
					<>
						{item}
						{index < array.length - 1 && (
							<SingleSimpleSeparator direction="vertical" />
						)}
					</>
				))}
		</div>
	);
}
"#;

pub const AUTH_NAV: UiComponent = UiComponent {
    name: "auth-nav",
    filename: "AuthNav.tsx",
    contents: AUTH_NAV_LIT,
    description: "Navigation for auth - switch between Signup and Login, or Reset Password",
    long_description: "Switches view in state, no page refresh :)",
    folder_path: "features/auth",
    npm_deps: &[],
};
