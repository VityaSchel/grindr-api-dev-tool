import { api, type AccountInfo, type DeviceInfo } from "./api";

/**
 * Reactive store of authenticated accounts and the currently active one.
 * `activeId === null` is the unauthorized state (only no-auth endpoints work).
 */
class AccountsStore {
	accounts = $state<AccountInfo[]>([]);
	activeId = $state<string | null>(null);
	loaded = $state(false);
	error = $state<string | null>(null);

	get active(): AccountInfo | null {
		return this.accounts.find((a) => a.id === this.activeId) ?? null;
	}

	async load() {
		try {
			const [accounts, activeId] = await Promise.all([
				api.listAccounts(),
				api.getActive(),
			]);
			this.accounts = accounts;
			this.activeId = activeId;
			this.error = null;
		} catch (e) {
			this.error = String(e);
			this.accounts = [];
			this.activeId = null;
			console.error("failed to load accounts", e);
		} finally {
			this.loaded = true;
		}
	}

	async setActive(id: string | null) {
		await api.setActive(id);
		this.activeId = id;
	}

	async remove(id: string) {
		const newActive = await api.deleteAccount(id);
		this.accounts = this.accounts.filter((a) => a.id !== id);
		this.activeId = newActive;
	}

	async add(
		email: string,
		authToken: string,
		device: DeviceInfo | null,
	): Promise<AccountInfo> {
		const info = await api.addAccount(email, authToken, device);
		const idx = this.accounts.findIndex((a) => a.id === info.id);
		if (idx >= 0) this.accounts[idx] = info;
		else this.accounts.push(info);
		this.activeId = info.id;
		return info;
	}
}

export const accounts = new AccountsStore();
