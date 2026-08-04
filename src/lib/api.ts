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
};

export type AccountInfo = { id: string; email: string; profile_id: string };

export type SessionRestriction = string | Record<string, unknown>;

/** Rust `Session` — the stored credentials of an account. */
export type Session = {
	email: string;
	expires_at: number;
	profile_id: string;
	session_id: string;
	auth_token: string;
	kind: "Email" | "Google";
	third_party_user_id: string | null;
	restriction: SessionRestriction | null;
};

export type AccountDetails = { device: DeviceInfo; session: Session };

/** Credentials for a sign-in method, matching Rust's tagged `SignInCredentials`. */
export type SignInCredentials =
	| { method: "token"; email: string; authToken: string }
	| { method: "password"; email: string; password: string }
	| { method: "google"; googleToken: string };

export type ResponsePayload = { status: number; body: string };

export type FileMeta = { name: string; size: number };

export type BodyFile = { path: string; contentType: string; signed: boolean };

// Tauri v2 commands default to `ArgumentCase::Camel` (Rust `auth_token` = js `authToken`)
export const api = {
	generateDevice: () => invoke<DeviceInfo>("generate_device"),
	requiresSignature: (path: string) =>
		invoke<boolean>("requires_signature", { path }),
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
