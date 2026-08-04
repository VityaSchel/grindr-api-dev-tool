import { api, type ResponsePayload } from "./api";

export class RequestRunner {
	sending = $state(false);
	cancelled = $state(false);
	error = $state<string | null>(null);
	response = $state<ResponsePayload | null>(null);
	elapsed = $state<number | null>(null);
	#id = "";

	get started(): boolean {
		return (
			this.sending ||
			this.response !== null ||
			this.error !== null ||
			this.cancelled
		);
	}

	async run(send: (requestId: string) => Promise<ResponsePayload>) {
		if (this.sending) return;
		this.sending = true;
		this.cancelled = false;
		this.error = null;
		this.response = null;
		this.elapsed = null;
		this.#id = crypto.randomUUID();
		const start = performance.now();
		try {
			this.response = await send(this.#id);
		} catch (e) {
			// A user-initiated cancel rejects too; show it as neutral, not an error.
			if (!this.cancelled) this.error = String(e);
		} finally {
			this.elapsed = Math.round(performance.now() - start);
			this.sending = false;
		}
	}

	fail(message: string) {
		this.response = null;
		this.elapsed = null;
		this.cancelled = false;
		this.error = message;
	}

	cancel() {
		if (!this.sending) return;
		this.cancelled = true;
		void api.cancelRequest(this.#id);
	}
}
