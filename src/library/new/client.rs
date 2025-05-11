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
    pub args: &'static [&'static str]
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
		"clsx": "^2.1.1",
		"solid-js": "^1.9.5",
		"tailwind-merge": "^3.2.0",
		"valibot": "^0.29.0",
		"vinxi": "^0.5.3"
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
import { Suspense } from "solid-js";
import "./app.css";

export default function App() {
	return (
		<Router
			root={(props) => (
				<MetaProvider>
					<Title>SolidStart - Basic</Title>
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
r#"import { mount, StartClient } from "@solidjs/start/client";

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

pub fn make_files() -> (Vec<NewDevanoClientFileInstruction>, Vec<NewDevanoClientBashInstruction>) {
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
      name: "index",
      filename: "index.tsx",
      folder_tree: &[".", "src", "routes"],
      contents: APP_TSX_LITERAL,
    },
  ];
  let commands = vec![
    NewDevanoClientBashInstruction {
      name: "pnpm_install",
      command: "pnpm",
      args: &["install"],
    },
  ];
  (files, commands)
}