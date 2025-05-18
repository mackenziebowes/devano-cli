use crate::library::client::components::atoms::registry::Atoms;
use crate::library::client::components::atoms::{
    buttons::Buttons, decorators::Decorators, feedback::Feedbacks, icons::Icons, inputs::Inputs,
    layout::Layouts, utils::Utils,
};
use crate::library::client::components::features::utils::{Installable, MoleculeInstallable};
use crate::library::client::utils::utils::{UiComponent, write_file};
use crate::library::client::{utils, writes};
use anyhow::Result;
use std::fs;
use std::io::{self};
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum AuthMolecules {
    Auth,
    AuthState,
    AuthInner,
    AuthNav,
    LogInForm,
    RegisterForm,
}

/// Implementation of the `AuthMolecules` enum, providing methods for managing
/// UI components and their dependencies in the authentication module.
///
/// # Methods
///
/// ## `atom_dependencies`
/// Returns a static reference to an array of `Atoms` that represent the dependencies
/// required by the specific `AuthMolecules` variant. Each variant has its own set of
/// dependencies, which may include buttons, inputs, or decorators.
///
/// ## `get_ui`
/// Returns a static reference to a `UiComponent` associated with the specific
/// `AuthMolecules` variant. This is used to retrieve the UI component's metadata,
/// such as its folder path, filename, and contents.
///
/// ## `install`
/// Installs the UI component represented by the `AuthMolecules` variant. This includes:
/// - Resolving the base path for the component.
/// - Recursively installing all atom dependencies.
/// - Checking if the required npm dependencies are installed.
/// - Writing the component's file to the appropriate location.
///
/// # Errors
/// The `install` method returns a `Result` that may contain an error if:
/// - A dependency installation fails.
/// - Npm dependencies are not installed.
/// - Writing the component file fails.
impl AuthMolecules {
    pub fn atom_dependencies(&self) -> &'static [Atoms] {
        match self {
            AuthMolecules::Auth => &[],
            AuthMolecules::AuthState => &[],
            AuthMolecules::AuthInner => &[],
            AuthMolecules::AuthNav => &[
                Atoms::Decorators(Decorators::Separators),
                Atoms::Buttons(Buttons::AnchorButton),
            ],
            // import { Button } from "~/devano/atoms/buttons/Button";
            // import { TextInput } from "~/devano/atoms/inputs/TextInput";
            // import { PasswordInput } from "~/devano/atoms/inputs/PasswordInput";
            // import { AuthNav } from "~/devano/features/auth/AuthNav";
            // import { Card } from "~/devano/atoms/layout/Card";
            // import { Heading } from "~/devano/workshop/Heading";
            // import { SimpleSeparator } from "~/devano/atoms/decorators/Separator";
            // import { api } from "~/devano/api";
            // import { ErrorMessage } from "~/devano/workshop/ErrorMessage";
            // import { useAuth } from "./AuthState";
            AuthMolecules::LogInForm => &[
                Atoms::Buttons(Buttons::Button),
                Atoms::Inputs(Inputs::Text),
                Atoms::Inputs(Inputs::Password),
                Atoms::Layout(Layouts::Card),
                Atoms::Feedback(Feedbacks::ErrorMsg),
                Atoms::Decorators(Decorators::Separators),
            ],
            AuthMolecules::RegisterForm => &[
                Atoms::Buttons(Buttons::Button),
                Atoms::Inputs(Inputs::Text),
                Atoms::Inputs(Inputs::Password),
                Atoms::Decorators(Decorators::Separators),
            ],
        }
    }

    pub fn get_ui(&self) -> &'static utils::utils::UiComponent {
        match self {
            AuthMolecules::Auth => &AUTH,
            AuthMolecules::AuthState => &AUTH_STATE,
            AuthMolecules::AuthInner => &AUTH_INNER,
            AuthMolecules::AuthNav => &AUTH_NAV,
            AuthMolecules::LogInForm => &LOG_IN_FORM,
            AuthMolecules::RegisterForm => &REGISTER_FORM,
        }
    }
}

impl Installable for AuthMolecules {
    fn name(&self) -> &str {
        let ui = self.get_ui();
        ui.name
    }

    fn install(&self) -> Result<()> {
        let base_path = Path::new("client/src/devano");
        let ui_component = self.get_ui();
        let component_path = base_path.join(ui_component.folder_path); // Use folder_path

        // Recursively install dependencies
        for dependency in self.atom_dependencies() {
            dependency.install()?; // Recursively call `install` for each dependency
        }

        // // Handle npm dependencies
        writes::npm::check_if_deps_installed(ui_component.npm_deps)?;

        let file_path = component_path.join(ui_component.filename); // Chain folder_path and filename

        write_file(&file_path, ui_component.contents)?;

        Ok(())
    }
}

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

pub const AUTH_STATE_LIT: &str = r#"
import { createSignal, createContext, useContext } from "solid-js";
import { z } from "zod";
import type { JSX, ParentProps, Accessor, Setter } from "solid-js";
import type { ZodType } from "zod";

export enum ViewState {
	LogIn = "log-in",
	Register = "register",
	ResetPasswordRequest = "reset-password-request",
	ResetPasswordChallenge = "reset-password-challenge",
	ResetPasswordFinal = "reset-password-final",
}

type StateControl = {
	get: Accessor<any>;
	set: Setter<any>;
};

type AuthContextType = {
	view: () => ViewState;
	setViewTo: {
		login: () => void;
		register: () => void;
		resetpasswordreq: () => void;
		resetpasswordchallenge: () => void;
		resetpasswordfinal: () => void;
	};
	state: {
		email: StateControl;
		password: StateControl;
		passwordChallenge: StateControl;
		error: StateControl;
	};
	schema: {
		email: ZodType;
		password: ZodType;
	};
	magic: {
		code_length: number;
	};
};

const AuthContext = createContext<AuthContextType>();

export const AuthProvider = (props: any) => {
	const emailSchema = z.object({
		email: z.string().email(),
	});
	const passwordSchema = z.object({
		password: z.string().min(8, "Password must be at least 8 characters long"),
	});
	const challenge_code_length = 6; // fml magic numbers - updates to the challenge code generator need you to update this magic number too
	const [view, setView] = createSignal<ViewState>(ViewState.LogIn);
	const [email, set_email] = createSignal<string>();
	const [password, set_password] = createSignal<string>();
	const [passwordChallenge, set_passwordChallenge] = createSignal();
	// challenge code intentionally missing from state control due to unique OTP impl of auto-sending upon code insertion
	const [error, set_error] = createSignal<string | null>();
	const setViewTo = {
		login: () => setView(ViewState.LogIn),
		register: () => setView(ViewState.Register),
		resetpasswordreq: () => setView(ViewState.ResetPasswordRequest),
		resetpasswordchallenge: () => setView(ViewState.ResetPasswordChallenge),
		resetpasswordfinal: () => setView(ViewState.ResetPasswordFinal),
	};
	const state = {
		email: {
			get: email,
			set: set_email,
		},
		password: {
			get: password,
			set: set_password,
		},
		passwordChallenge: {
			get: passwordChallenge,
			set: set_passwordChallenge,
		},
		error: {
			get: error,
			set: set_error,
		},
	};
	const schema = {
		email: emailSchema,
		password: passwordSchema,
	};
	const magic = {
		code_length: challenge_code_length,
	};
	return (
		<AuthContext.Provider value={{ view, setViewTo, state, schema, magic }}>
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
};
