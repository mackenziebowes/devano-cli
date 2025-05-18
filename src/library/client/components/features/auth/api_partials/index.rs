use crate::library::client::utils::utils::ApiComponent;

pub const INDEX_LIT: &str = r#"
import { z } from "zod";
import axios from "axios";
import { vona, useUserToken } from "~/devano/api/utils";

export const auth = {
	login,
	requestReset,
	register,
	submitResetChallenge,
};

const LoginPayload = z.object({
	email: z.string().email(),
	password: z.string(),
});

const LoginResponse = z.object({
	token: z.string(),
});

/**
 * Authenticates a user with the provided login payload.
 *
 * Sends a POST request to the `/auth/login` endpoint with the given credentials,
 * parses the response, stores the authentication token in local storage, and
 * updates the user token state via `useUserToken`.
 *
 * @param payload - The login credentials conforming to the `LoginPayload` schema.
 * @returns A promise that resolves to the parsed login response (`LoginResponse`) on success,
 * or an `Error` object if authentication fails or an unexpected error occurs.
 *
 * @throws {Error} If the request fails or the response is invalid.
 */
async function login(
	payload: z.infer<typeof LoginPayload>
): Promise<z.infer<typeof LoginResponse> | Error> {
	try {
		const response = await vona.post("/auth/login", payload);
		const data = LoginResponse.parse(response.data);
		localStorage.setItem("ut", data.token);
		// this is also set `onMount` at the top level
		useUserToken(() => localStorage.getItem("ut"));
		return data;
	} catch (error) {
		if (axios.isAxiosError(error)) {
			return new Error(
				error.response?.data?.message || "An error occurred during login"
			);
		}
		return new Error("An unexpected error occurred");
	}
}

const RequestResetPasswordPayload = z.object({
	email: z.string().email(),
});

const RequestResetResponse = z.object({
	accepted: z.boolean(),
});

/**
 * Sends a password reset request to the server with the provided payload.
 *
 * @param payload - The payload containing the necessary information to request a password reset, conforming to `RequestResetPasswordPayload`.
 * @returns A promise that resolves to the parsed response of type `RequestResetResponse` if successful, or an `Error` object if the request fails.
 *
 * @throws {Error} If an Axios error occurs, returns an `Error` with the server's message or a default message.
 * @throws {Error} If an unexpected error occurs, returns an `Error` with a generic message.
 */
async function requestReset(
	payload: z.infer<typeof RequestResetPasswordPayload>
): Promise<z.infer<typeof RequestResetResponse> | Error> {
	try {
		const response = await vona.post("/auth/reset-password", payload);
		return RequestResetResponse.parse(response.data);
	} catch (error) {
		if (axios.isAxiosError(error)) {
			return new Error(
				error.response?.data?.message ||
					"An error occurred during password reset"
			);
		}
		return new Error("An unexpected error occurred");
	}
}

// const ResetPasswordPayload = z.object({
// 	password: z.string(),
// });

const RegisterPayload = z.object({
	email: z.string().email(),
	password: z.string(),
});

const RegisterResponse = z.object({
	token: z.string(),
});

/**
 * Registers a new user with the provided payload.
 *
 * @param payload - The registration data conforming to the `RegisterPayload` schema.
 * @returns A promise that resolves to the parsed registration response (`RegisterResponse`) on success,
 * or an `Error` object if registration fails.
 *
 * @throws {Error} If an unexpected error occurs during the registration process.
 */
async function register(
	payload: z.infer<typeof RegisterPayload>
): Promise<z.infer<typeof RegisterResponse> | Error> {
	try {
		const response = await vona.post("/auth/register", payload);
		return RegisterResponse.parse(response.data);
	} catch (error) {
		if (axios.isAxiosError(error)) {
			return new Error(
				error.response?.data?.message || "An error occurred during registration"
			);
		}
		return new Error("An unexpected error occurred");
	}
}

const SubmitResetChallengePayload = z.object({
	code: z.string().length(6),
});

const SubmitResetChallengeResponse = z.object({
	success: z.boolean(),
});

/**
 * Submits a password reset challenge to the authentication API.
 *
 * @param payload - The payload containing the reset challenge data, validated by the `SubmitResetChallengePayload` schema.
 * @returns A promise that resolves to the parsed response conforming to the `SubmitResetChallengeResponse` schema,
 *          or an `Error` object if the request fails or the response is invalid.
 *
 * @throws {Error} If an Axios error occurs, returns an `Error` with a message from the response or a default message.
 *                 If an unexpected error occurs, returns a generic `Error`.
 */
async function submitResetChallenge(
	payload: z.infer<typeof SubmitResetChallengePayload>
): Promise<z.infer<typeof SubmitResetChallengeResponse> | Error> {
	try {
		const response = await vona.post("/auth/submit-reset-challenge", payload);
		return SubmitResetChallengeResponse.parse(response.data);
	} catch (error) {
		if (axios.isAxiosError(error)) {
			return new Error(
				error.response?.data?.message ||
					"An error occurred during password reset challenge submission"
			);
		}
		return new Error("An unexpected error occurred");
	}
}
"#;

pub const INDEX: ApiComponent = ApiComponent {
    module_name: "auth",
    description: "Default HTTP routes for auth",
    long_description: "",
    filename: "index.ts",
    contents: INDEX_LIT,
    folder_path: "auth",
};
