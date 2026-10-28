import type { SignInCredentials } from "$lib/api";

export type SignInForm = {
	method: SignInCredentials["method"];
	email: string;
	authToken: string;
	password: string;
	captchaToken: string;
	googleToken: string;
	facebookToken: string;
};

export function emptySignInForm(): SignInForm {
	return {
		method: "token",
		email: "",
		authToken: "",
		password: "",
		captchaToken: "",
		googleToken: "",
		facebookToken: "",
	};
}

export function toCredentials(form: SignInForm): SignInCredentials | null {
	switch (form.method) {
		case "token": {
			const email = form.email.trim();
			const authToken = form.authToken.trim();
			return email && authToken
				? { method: "token", email, authToken }
				: null;
		}
		case "password": {
			const email = form.email.trim();
			return email && form.password
				? {
						method: "password",
						email,
						password: form.password,
						captchaToken: form.captchaToken.trim() || null,
					}
				: null;
		}
		case "google": {
			const googleToken = form.googleToken.trim();
			return googleToken ? { method: "google", googleToken } : null;
		}
		case "facebook": {
			const facebookToken = form.facebookToken.trim();
			return facebookToken ? { method: "facebook", facebookToken } : null;
		}
	}
}
