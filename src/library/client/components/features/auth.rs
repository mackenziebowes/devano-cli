use std::fs;
use std::path::Path;
use std::io::{self};
use anyhow::Result;
use crate::library::client::{writes, utils};
use crate::library::client::components::atoms::registry::Atoms;
use crate::library::client::utils::utils::{UiComponent};

// Pattern Break
// Feature components are kind of referenced once per app
// So we can declare them locally to the registry rather than breaking them up.
// The count of components does not increase without bound, and there's no clean grouping logic,
// So into a big list they go.
#[derive(Debug, Clone, Copy)]
pub enum AuthMolecules {
    Auth,
    AuthState,
    AuthInner,
    AuthNav,
    LogInForm,
}

impl AuthMolecules {
    pub fn devano_dependencies(&self) -> &'static [Atoms] {
        match self {
            AuthMolecules::Auth => &[AuthMolecules::AuthState, AuthMolecules::AuthInner],
            AuthMolecules::AuthState => &[],
            AuthMolecules::AuthInner => &[AuthMolecules::AuthState, AuthMolecules::LogInForm, AuthMolecules::RegisterForm, AuthMolecules::ForgotPasswordForm, AuthMolecules::PasswordChallengeForm],
            AuthMolecules::AuthNav => &[AuthMolecules::AuthState, Atoms::Decorators(Decorators::Separators), Atoms::Buttons(Buttons::AnchorButton)],
            AuthMolecules::LogInForm => &[AuthMolecules::AuthNav, Atoms::Buttons(Buttons::Button), Atoms::Inputs(Inputs::TextInput), Atoms::Inputs(Inputs::PasswordInput), Atoms::Decorators(Decorators::Separators)],
            AuthMolecules::RegisterForm => &[AuthMolecules::AuthNav, Atoms::Buttons(Buttons::Button), Atoms::Inputs(Inputs::TextInput), Atoms::Inputs(Inputs::PasswordInput), Atoms::Decorators(Decorators::Separators)],
        }
    }
    
    pub fn get_ui(&self) -> &'static utils::utils::UiComponent {
        match self {
            AuthMolecules::Auth => &AUTH,
            AuthMolecules::AuthState => &AUTH_STATE,
            AuthMolecules::AuthInner => &AUTH_INNER,
            AuthMolecules::AuthNav => &AUTH_NAV,
        }
    }

    pub fn install(&self) -> Result<()> {
        let base_path = Path::new("client/src/devano");
        let ui_component = self.get_ui();
        let component_path = base_path.join(ui_component.folder_path); // Use folder_path
    
        // Recursively install dependencies
        for dependency in self.devano_dependencies() {
            dependency.install()?; // Recursively call `install` for each dependency
        }
    
        // Ensure the subfolder exists
        if !component_path.exists() {
            fs::create_dir_all(&component_path)?;
        }
    
        // // Handle npm dependencies
        writes::npm::check_if_deps_installed(ui_component.npm_deps)?;
    
        let file_path = component_path.join(ui_component.filename); // Chain folder_path and filename
    
        if !file_path.exists() {
            // Write the file if it doesn't already exist
            fs::write(&file_path, ui_component.contents)?;
            println!("{} written to {:?}", ui_component.filename, file_path);
        } else {
            println!("{} already exists at {:?}", ui_component.filename, file_path);
        }
    
        Ok(())
    }
}

pub const AUTH_LIT: &str = r#"
import { AuthProvider } from "~/devano/state/AuthState";
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
}

pub const AUTH_STATE_LIT: &str = r#"
import { createSignal, createContext, useContext } from "solid-js";
import type { JSX, ParentProps } from "solid-js";

export enum ViewState {
	LogIn = "log-in",
	Register = "register",
	ResetPasswordRequest = "reset-password-request",
	ResetPasswordChallenge = "reset-password-challenge",
}
type AuthContextType = {
	view: () => ViewState;
	setView: (state: ViewState) => void;
	setViewTo: {
		login: () => void;
		register: () => void;
		resetpasswordreq: () => void;
		resetpasswordchallenge: () => void;
	};
};

const AuthContext = createContext<AuthContextType>();

export const AuthProvider = (props: any) => {
	const [view, setView] = createSignal<ViewState>(ViewState.LogIn);
	const setViewTo = {
		login: () => setView(ViewState.LogIn),
		register: () => setView(ViewState.Register),
		resetpasswordreq: () => setView(ViewState.ResetPasswordRequest),
		resetpasswordchallenge: () => setView(ViewState.ResetPasswordChallenge),
	};
	return (
		<AuthContext.Provider value={{ view, setView, setViewTo }}>
			{props.children}
		</AuthContext.Provider>
	);
};

export const useAuth = () => {
	const context = useContext(AuthContext);
	if (!context) {
		throw new Error("useAuth must be used within an AuthProvider");
	}
	return context;
};
"#;

pub const AUTH_STATE: UiComponent = UiComponent {
    name: "auth-state",
    filename: "AuthState.tsx",
    contents: AUTH_STATE_LIT,
    description: "Context for handling Auth views",
	long_description: "",
	folder_path: "state",
	npm_deps: &[],
};

pub const AUTH_INNER_LIT: &str = r#"
import LogInForm from "~/devano/features/auth/LogInForm";
import RegisterForm from "~/devano/features/auth/RegisterForm";
import { Switch, Match } from "solid-js";
import { useAuth, ViewState } from "~/devano/state/AuthState";

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
					<div>Reset Password Request Form</div>
				</Match>
				<Match when={view() === ViewState.ResetPasswordChallenge}>
					<div>Reset Password Challenge Form</div>
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
}

pub const AUTH_NAV_LIT: &str = r#"
import { useAuth, ViewState } from "~/devano/state/AuthState";
import { Show } from "solid-js";
import { SingleSimpleSeparator } from "~/devano/atoms/decorators/Separator";
import { AnchorButton } from "~/devano/atoms/buttons/AnchorButton";

export default function AuthNav() {
	let { view, setViewTo } = useAuth();

	return (
		<div class="flex align-center w-full gap-[6px] justify-center">
			<Show when={view() !== ViewState.LogIn}>
				<AnchorButton onClick={setViewTo.login}>Log In</AnchorButton>
				<SingleSimpleSeparator direction="vertical" />
			</Show>
			<Show when={view() !== ViewState.Register}>
				<AnchorButton onClick={setViewTo.register}>Register</AnchorButton>
				<SingleSimpleSeparator direction="vertical" />
			</Show>
			<Show when={view() !== ViewState.ResetPasswordRequest}>
				<AnchorButton onClick={setViewTo.resetpasswordreq}>
					Reset Password
				</AnchorButton>
			</Show>
		</div>
	);
}
"#;

pub const AUTH_INNER: UiComponent = UiComponent {
    name: "auth-nav",
    filename: "AuthNav.tsx",
	contents: AUTH_NAV_LIT,
	description: "Navigation for auth - switch between Signup and Login, or Reset Password",
	long_description: "Switches view in state, no page refresh :)",
	folder_path: "features/auth",
	npm_deps: &[],
}

pub const LOG_IN_FORM_LIT: &str = r#"
import { Button } from "~/devano/atoms/buttons/Button";
import { TextInput } from "~/devano/atoms/inputs/TextInput";
import { PasswordInput } from "~/devano/atoms/inputs/PasswordInput";
import AuthNav from "~/devano/features/auth/AuthNav";
import { SimpleSeparator } from "~/devano/atoms/decorators/Separator";

export default function LogInForm() {
	return (
		<div class="flex flex-col border-[2px] border-(--gh-e) px-[24px] py-[12px] pb-[24px] rounded-[12px] gap-[24px]">
			<h2 class="text-[24px] font-[600]">Log In</h2>
			<div class="flex flex-col gap-[12px]">
				<TextInput
					label="Email"
					placeholder="Enter your email"
				/>
				<PasswordInput label="Password" />
				<Button>Log In</Button>
			</div>
			<SimpleSeparator
				direction={"horizontal"}
				label="OR"
			/>
			<AuthNav />
		</div>
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
}

pub const REGISTER_FORM_LIT: &str = r#"
import { Button } from "~/devano/atoms/buttons/Button";
import { TextInput } from "~/devano/atoms/inputs/TextInput";
import { PasswordInput } from "~/devano/atoms/inputs/PasswordInput";
import AuthNav from "~/devano/features/auth/AuthNav";
import { SimpleSeparator } from "~/devano/atoms/decorators/Separator";

export default function RegisterForm() {
	return (
		<div class="flex flex-col border-[2px] border-(--gh-e) px-[24px] py-[12px] pb-[24px] rounded-[12px] gap-[24px]">
			<h2 class="text-[24px] font-[600]">Register</h2>
			<div class="flex flex-col gap-[12px]">
				<TextInput
					label="Email"
					placeholder="Enter your email"
				/>
				<PasswordInput label="Password" />
				<PasswordInput label="Confirm Password" />
				<Button>Register</Button>
			</div>
			<SimpleSeparator
				direction={"horizontal"}
				label="OR"
			/>
			<AuthNav />
		</div>
	);
}
"#;

pub const REGISTER_FORM: UiComponent = UiComponent {
    name: "register-form",
    filename: "RegisterForm.tsx",
	contents: REGISTER_FORM_LIT,
	description: "Form for signing up",
	long_description: "Roll your own function to call backend for now :)",
	folder_path: "features/auth",
	npm_deps: &[],
}