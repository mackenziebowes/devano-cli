use crate::library::client::utils::utils::UiComponent;

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
