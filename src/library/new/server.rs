pub struct NewDevanoServerFileInstruction {
    #[allow(dead_code)]
    pub name: &'static str,
    pub filename: &'static str,
    pub folder_tree: &'static [&'static str],
    pub contents: &'static str,
}

pub struct NewDevanoServerBashInstruction {
    #[allow(dead_code)]
    pub name: &'static str,
    pub command: &'static str,
    pub args: &'static [&'static str],
}

const TSCONFIG_JSON_LITERAL: &str = r#"
{
	"include": ["src/**/*"],
	"compilerOptions": {
		"target": "es2016",
		"module": "commonjs",
		"outDir": "./dist",
		"baseUrl": "./src",
		"paths": {
			"@/*": ["*"]
		},
		"esModuleInterop": true,
		"forceConsistentCasingInFileNames": true,
		"strict": true,
		"skipLibCheck": true
	},
	"exclude": ["node_modules"]
}
"#;

const PACKAGE_JSON_LITERAL: &str = r#"
{
	"name": "express-starter",
	"version": "1.0.0",
	"description": "an express starter by me",
	"main": "index.js",
	"scripts": {
		"dev": "tsx watch src",
		"start": "tsx src",
		"test": "echo \"Error: no test specified\" && exit 1"
	},
	"author": "",
	"license": "ISC",
	"dependencies": {
		"@prisma/client": "^5.22.0",
		"@types/cors": "^2.8.17",
		"@types/express": "^5.0.0",
		"@types/jsonwebtoken": "^9.0.7",
		"@types/node": "^22.9.0",
		"@types/node-cron": "^3.0.11",
		"@types/ws": "^8.5.13",
		"cors": "^2.8.5",
		"dotenv": "^16.4.5",
		"express": "^4.21.1",
		"express-file-routing": "^3.0.3",
		"jsonwebtoken": "^9.0.2",
		"node-cron": "^3.0.3",
		"nodemon": "^3.1.7",
		"prisma": "^5.22.0",
		"ts-node": "^10.9.2",
		"tsx": "^4.19.2",
		"typescript": "^5.6.3",
		"ws": "^8.18.0",
		"zod": "^3.23.8"
	}
}
"#;

const GITIGNORE_LITERAL: &str = r#"
node_modules
.env
"#;

const ENV_EXAMPLE_LITERAL: &str = r#"
PORT=3050
JWT_SECRET=secret
"#;

const TYPES_D_TS_LITERAL: &str = r#"
declare namespace Express {
	export interface Request {
		userId?: string;
	}
}
"#;

const INDEX_LITERAL: &str = r#"
import { PORT, init } from "./utils/env";
init(); // kill app if no env
import createRouter from "express-file-routing";

import express from "express";
import cors from "cors";
import http from "http";
import { WSS } from "./lib/wss";
import cron from "node-cron";

(async () => {
	const app = express();
	app.use(cors());
	app.use(express.json());
	await createRouter(app); // file router for awesome route management
	app.use((err: any, req: any, res: any, next: any) => {
		console.error(err);
		res.status(500).json({ error: err });
	}); // error handling middleware
	const server = http.createServer(app);
	server.on("upgrade", async (request, socket, head) => {
		const token = request.url?.split("?token=")[1];
		if (!token) {
			socket.destroy();
			return;
		}
		WSS.handleUpgrade(request, socket, head, (ws) => {
			WSS.emit("connection", ws, request);
		});
	});
	server.listen(PORT, () => {
		console.log(`Server is running on port ${PORT}`);
	});

	// cron job example
	async function exampleCron() {
		console.log("running a task every 5 seconds");
		const now = new Date();
		console.log(now);
	}
	cron.schedule("*/5 * * * * *", exampleCron);
})();
"#;

const TOKEN_LITERAL: &str = r#"
import jwt from "jsonwebtoken";
import { JWT_SECRET } from "@/utils/env";

export function generateTokens(userId: string) {
	const httpToken = jwt.sign({ userId }, JWT_SECRET, {
		expiresIn: "30d",
	});

	const wsToken = jwt.sign({ wsUserId: userId }, JWT_SECRET, {
		expiresIn: "30d",
	});

	return {
		token: httpToken,
		wsToken,
	};
}

export function verifyToken(token: string, type: "http" | "ws" = "http") {
	let userId = null;
	try {
		let payload = jwt.verify(token, JWT_SECRET);

		switch (type) {
			case "http":
				userId = (payload as { userId: string }).userId || null;
				break;
			case "ws":
				userId = (payload as { wsUserId: string }).wsUserId || null;
				break;
			default:
				break;
		}
	} catch {}
	return userId;
}
"#;

const WS_LITERAL: &str = r#"
import { OPEN, Server, type WebSocket } from "ws";
import { verifyToken } from "./token";

const connectedUsers = new Map<string, { clients: WebSocket[] }>();

export const get_connected_users = () => {
	// list the keys of the connectedUsers
	const connectedUsersArray = Array.from(connectedUsers.values());
	return connectedUsersArray;
};

export const WSS = new Server({ noServer: true });

WSS.on("connection", async (ws, request) => {
	const token = request.url!.split("?token=")[1];
	if (!token) {
		ws.close();
		return;
	}
	const userId = verifyToken(token, "ws");
	if (!userId) {
		ws.close();
		return;
	}
	console.log(`[ws]: User ${userId} connected`);
	ws.on("close", () => {
		console.log(`[ws]: User ${userId} disconnected`);
		const oldConnectedUser = connectedUsers.get(userId);
		if (oldConnectedUser)
			connectedUsers.set(userId, {
				clients:
					oldConnectedUser.clients.filter((client) => client !== ws) || [],
			});
	});
});

// parse ws results on the front end to manage UI locations + styles via type
// do whatever u want here
type WsMessageArgs = {
	message?: string;
	type?: string;
};

export function sendMessageToUser({
	user_id,
	args,
}: {
	user_id: string;
	args: WsMessageArgs;
}) {
	connectedUsers.get(user_id)?.clients.forEach((clientWs) => {
		if (clientWs.readyState === OPEN) {
			clientWs.send(JSON.stringify(args));
		}
	});
}
"#;

const AUTHENTICATE_REQUEST_LITERAL: &str = r#"
import { NextFunction, Request, Response } from "express";
import { verifyToken } from "@/lib/token";

// weird lil hack for later - see routes/example.ts for use
export default () =>
	async (req: Request, res: Response, next: NextFunction) => {
		const token = req.headers.authorization;
		if (!token) {
			// early return to save cycles :3
			res.status(401).json({ error: "Unauthorized" });
			return;
		}
		const userId = verifyToken(token, "http");
		if (!userId) {
			res.status(401).json({ error: "Unauthorized" });
			return;
		}
		// see types.d.ts for this "trick"
		req.userId = userId;
		// basically, add a userId to the request object so we know
		// 1. that we know you
		// 2. who you are
		next();
	};
"#;

const VALIDATE_SCHEMA_LITERAL: &str = r#"
import { Request, Response, NextFunction } from "express";
import { ZodSchema, ZodError } from "zod";

export default function validateSchema<t>(schema: ZodSchema<t>) {
	return async (req: Request, res: Response, next: NextFunction) => {
		try {
			schema.parse(req.body);
			next();
		} catch (error) {
			if (error instanceof ZodError) {
				// theoretically we log these errors if you want idk up to you
				// in this startpark we just ping it back to the client, not like they'd know what to do with it
				res.status(400).json({ error: error.errors });
			} else {
				res.status(500).json({ error: "Internal Server Error" });
			}
		}
	};
}
"#;

const EXAMPLE_ROUTE_LITERAL: &str = r#"
// this file is available at `${url}/example`
// it can have multiple exports
import authenticateRequest from "@/middleware/authenticateRequest";
import validateSchema from "@/middleware/validateSchema";
import { Request, Response, NextFunction } from "express";
import { z } from "zod";

export const get = [
	authenticateRequest(),
	async (req: Request, res: Response, next: NextFunction) => {
		// this route is protected by the authenticateRequest middleware
		// req also has a userId property
		res.json({ message: `Hello, ${req.userId}!` });
	},
];

// the lifetime of input types is typically identical to the route,
// so in my starterpack, I put them in the same file

const pingPongSchema = z.object({
	message: z.string(),
});

export const post = [
	authenticateRequest(),
	validateSchema(pingPongSchema),
	async (req: Request, res: Response, next: NextFunction) => {
		// this route is protected by the authenticateRequest middleware
		res.json({ message: `Pong! ${req.body.message}` });
	},
];
"#;

const ENV_LITERAL: &str = r#"
import dotenv from "dotenv";
dotenv.config();

// This is epic - run this code once and it will set the environment variables as variables so no dotenv barg
// also it will throw an error if the environment variables are not set

export const PORT = process.env.PORT + "";
export const JWT_SECRET = process.env.JWT_SECRET + "";

export function init() {
	if (PORT.length === 0) throw new Error("PORT is not set");
	if (JWT_SECRET.length === 0) throw new Error("JWT_SECRET is not set");
}
"#;

pub fn make_files() -> (
    Vec<NewDevanoServerFileInstruction>,
    Vec<NewDevanoServerBashInstruction>,
) {
    let files = vec![
        NewDevanoServerFileInstruction {
            name: "tsconfig",
            filename: "tsconfig.json",
            folder_tree: &["."],
            contents: TSCONFIG_JSON_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "package_json",
            filename: "package.json",
            folder_tree: &["."],
            contents: PACKAGE_JSON_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "git_ignore",
            filename: ".gitignore",
            folder_tree: &["."],
            contents: GITIGNORE_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "env_example",
            filename: ".env.example",
            folder_tree: &["."],
            contents: ENV_EXAMPLE_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "types",
            filename: "types.d.ts",
            folder_tree: &[".", "src"],
            contents: ENV_EXAMPLE_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "types",
            filename: "types.d.ts",
            folder_tree: &[".", "src"],
            contents: ENV_EXAMPLE_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "types",
            filename: "types.d.ts",
            folder_tree: &[".", "src"],
            contents: TYPES_D_TS_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "index",
            filename: "index.ts",
            folder_tree: &[".", "src"],
            contents: INDEX_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "lib/token",
            filename: "token.ts",
            folder_tree: &[".", "src", "lib"],
            contents: TOKEN_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "lib/ws",
            filename: "ws.ts",
            folder_tree: &[".", "src", "lib"],
            contents: WS_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "middleware/authenticateRequest",
            filename: "authenticateRequest.ts",
            folder_tree: &[".", "src", "middleware"],
            contents: AUTHENTICATE_REQUEST_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "middleware/validateSchema",
            filename: "validateSchema.ts",
            folder_tree: &[".", "src", "middleware"],
            contents: VALIDATE_SCHEMA_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "routes/example",
            filename: "example.ts",
            folder_tree: &[".", "src", "routes"],
            contents: EXAMPLE_ROUTE_LITERAL,
        },
        NewDevanoServerFileInstruction {
            name: "utils/env",
            filename: "env.ts",
            folder_tree: &[".", "src", "utils"],
            contents: ENV_LITERAL,
        },
    ];
    let commands = vec![
        NewDevanoServerBashInstruction {
            name: "git/init",
            command: "git",
            args: &["init"],
        },
        NewDevanoServerBashInstruction {
            name: "pnpm/i",
            command: "pnpm",
            args: &["install"],
        },
    ];
    (files, commands)
}
