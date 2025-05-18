use crate::library::client::utils::utils::UiComponent;

pub const AUTH_LIT: &str = r#"
import { AuthProvider } from "~/devano/features/auth/AuthState";
import AuthInner from "~/devano/features/auth/AuthInner";

export default function Auth() {
	return (
		<div>
			<AuthProvider>
				<AuthInner />
			</AuthProvider>
		</div>
	);
}
"#;

pub const AUTH: UiComponent = UiComponent {
    name: "auth",
    filename: "Auth.tsx",
    contents: AUTH_LIT,
    description: "The main Auth component, drop this in a route or modal to have auth be there.",
    long_description: "Imports and wraps AuthInner with AuthProvider, enabling internal state.",
    folder_path: "features/auth",
    npm_deps: &[],
};
