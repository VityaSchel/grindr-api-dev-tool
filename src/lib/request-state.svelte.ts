/**
 * Tracks whether the currently shown request has unsaved edits. When dirty,
 * internal navigations (links to types or other endpoints) open in a new window
 * instead of replacing the in-progress request — see `beforeNavigate` in the
 * root layout.
 */
class RequestState {
	dirty = $state(false);

	markDirty() {
		this.dirty = true;
	}

	reset() {
		this.dirty = false;
	}
}

export const requestState = new RequestState();
