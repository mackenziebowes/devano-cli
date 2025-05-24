pub struct NewDevanoClientFileInstruction {
    #[allow(dead_code)]
    pub name: &'static str,
    pub filename: &'static str,
    pub folder_tree: &'static [&'static str],
    pub contents: &'static str,
}

pub struct NewDevanoClientBashInstruction {
    #[allow(dead_code)]
    pub name: &'static str,
    pub command: &'static str,
    pub args: &'static [&'static str],
}

const TSCONFIG_LITERAL: &str = r#"
    {
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true,
    "jsx": "preserve",
    "jsxImportSource": "solid-js",
    "allowJs": true,
    "strict": true,
    "noEmit": true,
    "types": ["vinxi/types/client"],
    "isolatedModules": true,
    "paths": {
      "~/*": ["./src/*"]
    }
  }
}
"#;

const APP_CONFIG_LITERAL: &str = r#"
    import { defineConfig } from "@solidjs/start/config";
import tailwindcss from "@tailwindcss/vite";
export default defineConfig({
	vite: {
		plugins: [tailwindcss()],
	},
});
"#;

const PACKAGE_JSON_LITERAL: &str = r#"
{
	"name": "Devano Web App",
	"type": "module",
	"scripts": {
		"dev": "vinxi dev",
		"build": "vinxi build",
		"start": "vinxi start"
	},
	"dependencies": {
		"@solidjs/meta": "^0.29.4",
		"@solidjs/router": "^0.15.0",
		"@solidjs/start": "^1.1.0",
		"@trpc/client": "^10.45.2",
		"@trpc/server": "^10.45.2",
		"@typeschema/valibot": "^0.13.4",
		"axios": "^1.9.0",
		"clsx": "^2.1.1",
		"solid-js": "^1.9.5",
		"tailwind-merge": "^3.2.0",
		"valibot": "^0.29.0",
		"vinxi": "^0.5.3",
		"zod": "3.24.2"
	},
	"devDependencies": {
		"@tailwindcss/vite": "^4.0.7",
		"tailwindcss": "^4.0.7"
	},
	"engines": {
		"node": ">=22"
	}
}
"#;

const GITIGNORE_LITERAL: &str = r#"
dist
.wrangler
.output
.vercel
.netlify
.vinxi
app.config.timestamp_*.js

# Environment
.env
.env*.local

# dependencies
/node_modules

# IDEs and editors
/.idea
.project
.classpath
*.launch
.settings/

# Temp
gitignore

# System Files
.DS_Store
Thumbs.db
"#;

const APP_TSX_LITERAL: &str = r#"
import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense, onMount } from "solid-js";
import { api } from "~/devano/api";
import "./app.css";
import "./css/devano/palette.css";

export default function App() {
	onMount(() => {
		// utils for validating sessions and users
		api.tokens.user(() => localStorage.getItem("ut"));
		api.tokens.handshake(() => localStorage.getItem("hs"));
		if (!localStorage.getItem("hs")) {
			// do the handshake
			api.handshake();
		}
	});
	return (
		<Router
			root={(props) => (
				<MetaProvider>
					<Title>Devano</Title>
					<Suspense>{props.children}</Suspense>
				</MetaProvider>
			)}
		>
			<FileRoutes />
		</Router>
	);
}
"#;

const ENTRY_CLIENT_LITERAL: &str = r#"
import { mount, StartClient } from "@solidjs/start/client";

mount(() => <StartClient />, document.getElementById("app")!);
"#;

const ENTRY_SERVER_LITERAL: &str = r#"
// @refresh reload
import { createHandler, StartServer } from "@solidjs/start/server";

export default createHandler(() => (
  <StartServer
    document={({ assets, children, scripts }) => (
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="icon" href="/favicon.ico" />
          {assets}
        </head>
        <body>
          <div id="app">{children}</div>
          {scripts}
        </body>
      </html>
    )}
  />
));
"#;

const ENV_LIT: &str = r#"
VITE_API_URL=""
"#;

const ENV_D_TS_LIT: &str = r#"
interface ImportMetaEnv {
	readonly VITE_API_URL: string;
}
"#;

const API_UTILS_LIT: &str = r#"
/**
 * Utility module for API interactions using Axios.
 *
 * This module provides two Axios instances, `sona` and `vona`, for handling API requests
 * with different levels of authorization. It also includes mechanisms for dynamically
 * injecting user and handshake tokens into requests made with the `vona` instance.
 *
 * ### Exports:
 *
 * - `BASE_URL`: The base URL for API requests, derived from environment variables.
 * - `sona`: An Axios instance for public API access, configured with cookies.
 * - `vona`: An Axios instance for authenticated API access, configured with cookies and
 *   optional Authorization and handshake tokens.
 * - `useUserToken(fn: () => MaybeString)`: A function to set the user token retrieval mechanism
 *   for the `vona` instance.
 * - `useHandshakeToken(fn: () => MaybeString)`: A function to set the handshake token retrieval mechanism
 *   for the `vona` instance.
 *
 * ### Axios Instances:
 *
 * - **`sona`**:
 *   - Represents a "stranger" or public access level.
 *   - Does not include Authorization headers.
 *   - Uses cookies for session management.
 *   - Includes a rate-limiting mechanism to prevent excessive requests.
 *
 * - **`vona`**:
 *   - Represents a "trusted" or authenticated access level.
 *   - Dynamically includes an Authorization header with a Bearer token if provided.
 *   - Dynamically includes a handshake token in the `ui-access-key` header if provided.
 *   - Uses cookies for session management.
 *
 * ### Functions:
 *
 * - **`useUserToken(fn: () => MaybeString)`**:
 *   - Sets a callback function to retrieve the user authentication token.
 *   - The token is dynamically injected into the Authorization header of requests made
 *     with the `vona` instance.
 *
 * - **`useHandshakeToken(fn: () => MaybeString)`**:
 *   - Sets a callback function to retrieve the handshake token.
 *   - The token is dynamically injected into the `ui-access-key` header of requests made
 *     with the `vona` instance.
 *
 * ### Interceptors:
 *
 * - The `sona` instance includes a request interceptor that:
 *   - Enforces a minimum time interval (5 seconds) between consecutive requests.
 *   - Rejects requests made too frequently with an appropriate error message.
 *
 * - The `vona` instance includes a request interceptor that:
 *   - Retrieves the user and handshake tokens using the callback functions set by
 *     `useUserToken` and `useHandshakeToken`.
 *   - Appends the tokens as headers (`Authorization` and `ui-access-key`) if available.
 */

import axios from "axios";

export const BASE_URL = import.meta.env.VITE_API_URL;

/**
 * "Sona" is a Diralevan (conlang) reconstruction of PIE "sen-"" which,
 * in English, has descended into "senior" and "senate" - it means "outsider" in a neutral way.
 * This value, `sona`, is a plain axios instance for handling API calls without passing
 * any kind of tokens beyond standard cookies.
 */
export const sona = axios.create({
	// sona = stranger, public access
	baseURL: BASE_URL,
	withCredentials: true,
});

/**
 * "Vona" is a Diralevan (conlang) reconstruction of PIE 'wen-' which,
 * in English, has descended into "friend".
 * This value, `vona`, is an axios instance that appends a user-level token
 * for Authorization and a session level token for managing requests.
 */
export const vona = axios.create({
	// vona = trusted / known... bit of wishful thinking to trust a client you know
	baseURL: BASE_URL,
	withCredentials: true,
});

type MaybeString = string | undefined | null;

let getUserToken: (() => MaybeString) | null = null;
let getHandshakeToken: (() => MaybeString) | null = null;

export function useUserToken(fn: () => MaybeString) {
	getUserToken = fn;
}
export function useHandshakeToken(fn: () => MaybeString) {
	getHandshakeToken = fn;
}

let lastSonaRequest: Date | null = null;

sona.interceptors.request.use((config) => {
	const now = new Date();
	if (lastSonaRequest) {
		// Calculate the time difference in milliseconds
		const timeSinceLastRequest = now.getTime() - lastSonaRequest.getTime();

		// If the time difference is less than 5 seconds (5000 ms), reject the request
		// yet another magic number, fml
		if (timeSinceLastRequest < 5000) {
			return Promise.reject({
				message: "Too many requests. Please wait before trying again.",
			});
		}
	}
	lastSonaRequest = now;
	return config;
});

vona.interceptors.request.use((config) => {
	const userToken = getUserToken?.();
	const handshakeToken = getHandshakeToken?.();
	if (userToken) {
		config.headers.Authorization = `Bearer ${userToken}`;
		config.headers["ui-access-key"] = `Bearer ${handshakeToken}`;
	}
	return config;
});
"#;

pub const API_INDEX_LIT: &str = r#"
import { handshake } from "./handshake";
import { useUserToken, useHandshakeToken } from "./utils";

export const api = {
	handshake,
	tokens: {
		user: useUserToken,
		handshake: useHandshakeToken,
	},
};
"#;

pub const API_HANDSHAKE_LIT: &str = r#"
import axios from "axios";
import { sona, useHandshakeToken } from "~/devano/api/utils";
import { z } from "zod";

const HandshakeResponse = z.object({
	token: z.string(),
});

export async function handshake() {
	try {
		const response = await sona.get("/hs");
		const data = HandshakeResponse.parse(response.data);
		localStorage.setItem("hs", data.token);
    // this is also set `onMount` at the top level
		useHandshakeToken(() => localStorage.getItem("hs"));
	} catch (error) {
		if (axios.isAxiosError(error)) {
			return new Error(
				error.response?.data?.message || "An error occurred during login"
			);
		}
		return new Error("An unexpected error occurred");
	}
}
"#;

pub fn make_files() -> (
    Vec<NewDevanoClientFileInstruction>,
    Vec<NewDevanoClientBashInstruction>,
) {
    let files = vec![
        NewDevanoClientFileInstruction {
            name: "tsconfig",
            filename: "tsconfig.json",
            folder_tree: &["."],
            contents: TSCONFIG_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "appconfig",
            filename: "app.config.ts",
            folder_tree: &["."],
            contents: APP_CONFIG_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "packageJson",
            filename: "package.json",
            folder_tree: &["."],
            contents: PACKAGE_JSON_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "gitignore",
            filename: ".gitignore",
            folder_tree: &["."],
            contents: GITIGNORE_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "global-types",
            filename: "global.d.ts",
            folder_tree: &[".", "src"],
            contents: r#"/// <reference types="@solidjs/start/env" />"#,
        },
        NewDevanoClientFileInstruction {
            name: "entry-client",
            filename: "entry-client.tsx",
            folder_tree: &[".", "src"],
            contents: ENTRY_CLIENT_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "entry-server",
            filename: "entry-server.tsx",
            folder_tree: &[".", "src"],
            contents: ENTRY_SERVER_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "app_css",
            filename: "app.css",
            folder_tree: &[".", "src"],
            contents: r#"@import "tailwindcss";"#,
        },
        NewDevanoClientFileInstruction {
            name: "app_tsx",
            filename: "app.tsx",
            folder_tree: &[".", "src"],
            contents: APP_TSX_LITERAL,
        },
        NewDevanoClientFileInstruction {
            name: "env",
            filename: ".env",
            folder_tree: &["."],
            contents: ENV_LIT,
        },
        NewDevanoClientFileInstruction {
            name: "env-declaration",
            filename: "env.d.ts",
            folder_tree: &["."],
            contents: ENV_D_TS_LIT,
        },
        NewDevanoClientFileInstruction {
            name: "api/utils",
            filename: "utils.ts",
            folder_tree: &[".", "src", "devano", "api"],
            contents: API_UTILS_LIT,
        },
        NewDevanoClientFileInstruction {
            name: "api/index",
            filename: "index.ts",
            folder_tree: &[".", "src", "devano", "api"],
            contents: API_INDEX_LIT,
        },
        NewDevanoClientFileInstruction {
            name: "api/handshake",
            filename: "index.ts",
            folder_tree: &[".", "src", "devano", "api", "handshake"],
            contents: API_HANDSHAKE_LIT,
        },
    ];
    let commands = vec![NewDevanoClientBashInstruction {
        name: "pnpm_install",
        command: "pnpm",
        args: &["install"],
    }];
    (files, commands)
}
