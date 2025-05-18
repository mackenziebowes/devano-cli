use crate::library::client::utils::utils::UiComponent;

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
