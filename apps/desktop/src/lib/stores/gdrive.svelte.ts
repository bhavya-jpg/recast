import { isTauriApp } from "$lib/runtime/tauri";

/**
 * Google Drive store.
 *
 * Mirrors the shape of {@link import("./updater.svelte").updater} — a small
 * `$state`-backed module singleton that the UI binds to directly. Holds the
 * current connection state, the connected account's email (best-effort
 * populated from Google's userinfo endpoint), and an `uploads` map keyed by
 * `uploadId`. The store is a thin shell over Tauri commands and events; the
 * actual OAuth + Drive REST plumbing lives in `commands/gdrive.rs`.
 *
 * Lazy imports keep this module safe to load in the web build, where the
 * Tauri runtime doesn't exist.
 */

export type GdriveUploadStatus = "uploading" | "complete" | "error" | "cancelled";

export type GdriveUpload = {
	uploadId: string;
	fileName: string;
	bytesSent: number;
	totalBytes: number;
	status: GdriveUploadStatus;
	webViewLink?: string;
	error?: string;
};

type GdriveUploadResult = {
	fileId: string;
	name: string;
	webViewLink?: string;
};

function createGdriveStore() {
	let connected = $state(false);
	let email = $state<string | null>(null);
	let connecting = $state(false);
	const uploads = $state<Record<string, GdriveUpload>>({});

	let listenersAttached = false;

	async function attachListeners() {
		if (listenersAttached) return;
		if (!(await isTauriApp())) return;
		listenersAttached = true;
		const { listen } = await import("@tauri-apps/api/event");

		await listen<{ connected: boolean; email?: string | null }>(
			"gdrive:connected",
			({ payload }) => {
				connected = payload.connected;
				email = payload.email ?? null;
				connecting = false;
			},
		);
		await listen<{
			uploadId: string;
			bytesSent: number;
			totalBytes: number;
		}>("gdrive:progress", ({ payload }) => {
			const existing = uploads[payload.uploadId];
			if (!existing) return;
			uploads[payload.uploadId] = {
				...existing,
				bytesSent: payload.bytesSent,
				totalBytes: payload.totalBytes,
			};
		});
		await listen<{ uploadId: string } & GdriveUploadResult>(
			"gdrive:upload-complete",
			({ payload }) => {
				const existing = uploads[payload.uploadId];
				if (!existing) return;
				uploads[payload.uploadId] = {
					...existing,
					status: "complete",
					bytesSent: existing.totalBytes || existing.bytesSent,
					webViewLink: payload.webViewLink,
				};
			},
		);
		await listen<{ uploadId: string; message: string; cancelled: boolean }>(
			"gdrive:upload-error",
			({ payload }) => {
				const existing = uploads[payload.uploadId];
				if (!existing) return;
				uploads[payload.uploadId] = {
					...existing,
					status: payload.cancelled ? "cancelled" : "error",
					error: payload.cancelled ? undefined : payload.message,
				};
			},
		);
	}

	/** Read current connection state from the Rust side. Best-effort. */
	async function refreshStatus() {
		if (!(await isTauriApp())) return;
		try {
			const { invoke } = await import("@tauri-apps/api/core");
			const status = await invoke<{ connected: boolean; email?: string }>(
				"gdrive_status",
			);
			connected = status.connected;
			email = status.email ?? null;
		} catch (e) {
			console.error("[gdrive] status check failed", e);
		}
	}

	/**
	 * Start the OAuth flow. The Rust side opens the browser, awaits the
	 * loopback callback, exchanges the code, persists the refresh token,
	 * and emits `gdrive:connected` on success. We just need to flip
	 * `connecting` while it's in flight.
	 */
	async function connect() {
		if (!(await isTauriApp())) return;
		await attachListeners();
		connecting = true;
		try {
			const { invoke } = await import("@tauri-apps/api/core");
			await invoke("gdrive_connect");
			// Success path: the `gdrive:connected` listener flips state.
		} catch (e) {
			connecting = false;
			console.error("[gdrive] connect failed", e);
			throw e;
		}
	}

	async function disconnect() {
		if (!(await isTauriApp())) return;
		try {
			const { invoke } = await import("@tauri-apps/api/core");
			await invoke("gdrive_disconnect");
		} catch (e) {
			console.error("[gdrive] disconnect failed", e);
		}
		connected = false;
		email = null;
	}

	/**
	 * Kick off an upload. Returns the synthetic upload id so callers can
	 * pair UI state (toast cards, progress bars) to the same key the
	 * store and Rust side use. The Promise resolves with the result or
	 * rejects on failure — but the corner-card UI usually relies on the
	 * `uploads` map updating via events, not on awaiting this Promise.
	 */
	async function upload(path: string): Promise<GdriveUploadResult> {
		if (!(await isTauriApp())) throw new Error("not running in Tauri");
		await attachListeners();
		const fileName = path.split(/[\\/]/).pop() ?? path;
		const uploadId = `upload-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
		uploads[uploadId] = {
			uploadId,
			fileName,
			bytesSent: 0,
			totalBytes: 0,
			status: "uploading",
		};
		const { invoke } = await import("@tauri-apps/api/core");
		try {
			return await invoke<GdriveUploadResult>("gdrive_upload", {
				path,
				uploadId,
			});
		} catch (e) {
			// The Rust side already emitted `gdrive:upload-error` for the
			// corner-card UI; re-throw for any caller that awaits the
			// Promise (e.g. for inline error toasts).
			throw e;
		}
	}

	async function cancelUpload(uploadId: string) {
		if (!(await isTauriApp())) return;
		const { invoke } = await import("@tauri-apps/api/core");
		try {
			await invoke("gdrive_cancel_upload", { uploadId });
		} catch (e) {
			console.error("[gdrive] cancel failed", e);
		}
	}

	function dismissUpload(uploadId: string) {
		delete uploads[uploadId];
	}

	return {
		get connected() {
			return connected;
		},
		get email() {
			return email;
		},
		get connecting() {
			return connecting;
		},
		get uploads() {
			return uploads;
		},
		get activeUploads() {
			return Object.values(uploads);
		},

		/** Wire event listeners and pull current status. Safe to call repeatedly. */
		async init() {
			await attachListeners();
			await refreshStatus();
		},

		refreshStatus,
		connect,
		disconnect,
		upload,
		cancelUpload,
		dismissUpload,
	};
}

export const gdrive = createGdriveStore();
