import { browser } from "$app/environment";

const STORAGE_KEY = "response-pane-height";
const DEFAULT_HEIGHT = 320;

export const MIN_RESPONSE_HEIGHT = 88;
export const MIN_REQUEST_HEIGHT = 140;

function restore(): number {
	if (!browser) return DEFAULT_HEIGHT;
	const saved = Number(localStorage.getItem(STORAGE_KEY));
	return saved >= MIN_RESPONSE_HEIGHT ? saved : DEFAULT_HEIGHT;
}

class ResponsePaneHeight {
	current = $state(restore());

	remember() {
		if (browser)
			localStorage.setItem(STORAGE_KEY, String(Math.round(this.current)));
	}
}

export const responsePane = new ResponsePaneHeight();
