import { invoke } from "@tauri-apps/api/core";

/** Rust `DeviceInfo` */
export type DeviceInfo = {
	device_type: number;
	device_id: string;
	os: string;
	screen_resolution: string;
	total_ram: string;
	advertising_id: string;
	device_model: string;
	manufacturer: string;
	timezone: string;
	locale: string;
	accept_language: string;
	build_id: string;
};

/** "Advanced" */
export const DEVICE_FIELD_LABELS: Record<keyof DeviceInfo, string> = {
	device_type: "Device type",
	device_id: "Device ID",
	os: "OS",
	screen_resolution: "Screen resolution",
	total_ram: "Total RAM",
	advertising_id: "Advertising ID",
	device_model: "Device model",
	manufacturer: "Manufacturer",
	timezone: "Timezone",
	locale: "Locale",
	accept_language: "Accept-Language",
	build_id: "Build ID",
};

export type AccountInfo = { id: string; email: string; profile_id: string };

export type SessionRestriction = string | Record<string, unknown>;

/** Rust `Credentials` — the stored half of an account's session. */
export type Credentials = {
	email: string;
	profile_id: string | null;
	auth_token: string;
	kind: "Email" | "Google" | "Facebook";
	third_party_user_id: string | null;
};

/** The short-lived half, known only while the account is active. */
export type SessionToken = {
	session_id: string;
	expires_at: number;
	restriction: SessionRestriction | null;
};

export type AccountDetails = {
	device: DeviceInfo;
	credentials: Credentials;
	token: SessionToken | null;
};

/** Credentials for a sign-in method, matching Rust's tagged `SignInCredentials`. */
export type SignInCredentials =
	| { method: "token"; email: string; authToken: string }
	| {
			method: "password";
			email: string;
			password: string;
			captchaToken: string | null;
	  }
	| { method: "google"; googleToken: string }
	| { method: "facebook"; facebookToken: string };

export type ResponsePayload = { status: number; body: string };

export type FileMeta = { name: string; size: number };

export type BodyFile = { path: string; contentType: string; signed: boolean };

// Tauri v2 commands default to `ArgumentCase::Camel` (Rust `auth_token` = js `authToken`)
export const api = {
	generateDevice: () => invoke<DeviceInfo>("generate_device"),
	fetchOpenapi: () => invoke<string>("fetch_openapi"),
	listAccounts: () => invoke<AccountInfo[]>("list_accounts"),
	getActive: () => invoke<string | null>("get_active"),
	addAccount: (
		credentials: SignInCredentials,
		device: DeviceInfo | null,
		geohash: string | null,
	) => invoke<AccountInfo>("add_account", { credentials, device, geohash }),
	accountDetails: (id: string) =>
		invoke<AccountDetails>("account_details", { id }),
	updateAccountDevice: (id: string, device: DeviceInfo) =>
		invoke<void>("update_account_device", { id, device }),
	setActive: (id: string | null) => invoke<void>("set_active", { id }),
	deleteAccount: (id: string) =>
		invoke<string | null>("delete_account", { id }),
	sendRequest: (
		method: string,
		path: string,
		body: unknown | null,
		requestId: string,
		bodyFile: BodyFile | null = null,
	) =>
		invoke<ResponsePayload>("send_request", {
			method,
			path,
			body,
			requestId,
			bodyFile,
		}),
	cancelRequest: (requestId: string) =>
		invoke<void>("cancel_request", { requestId }),
	statFile: (path: string) => invoke<FileMeta>("stat_file", { path }),
};
