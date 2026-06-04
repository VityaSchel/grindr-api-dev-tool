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

export type ResponsePayload = { status: number; body: string };

// Tauri v2 commands default to `ArgumentCase::Camel` (Rust `auth_token` = js `authToken`)
export const api = {
	generateDevice: () => invoke<DeviceInfo>("generate_device"),
	listAccounts: () => invoke<AccountInfo[]>("list_accounts"),
	getActive: () => invoke<string | null>("get_active"),
	addAccount: (email: string, authToken: string, device: DeviceInfo | null) =>
		invoke<AccountInfo>("add_account", { email, authToken, device }),
	setActive: (id: string | null) => invoke<void>("set_active", { id }),
	deleteAccount: (id: string) =>
		invoke<string | null>("delete_account", { id }),
	sendRequest: (method: string, path: string, body: unknown | null) =>
		invoke<ResponsePayload>("send_request", { method, path, body }),
};
