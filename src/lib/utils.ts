import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
	? Omit<T, "children">
	: T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
	ref?: U | null;
};

const MIME_BY_EXTENSION: Record<string, string> = {
	jpg: "image/jpeg",
	jpeg: "image/jpeg",
	png: "image/png",
	gif: "image/gif",
	webp: "image/webp",
	heic: "image/heic",
	heif: "image/heif",
	bmp: "image/bmp",
	svg: "image/svg+xml",
	mp4: "video/mp4",
	m4v: "video/x-m4v",
	mov: "video/quicktime",
	webm: "video/webm",
	m4a: "audio/mp4",
	mp3: "audio/mpeg",
	aac: "audio/aac",
	wav: "audio/wav",
	ogg: "audio/ogg",
	pdf: "application/pdf",
	json: "application/json",
	txt: "text/plain",
	gz: "application/gzip",
	zip: "application/zip",
};

export function guessMimeType(fileName: string): string | undefined {
	const ext = fileName.split(".").pop()?.toLowerCase();
	return ext ? MIME_BY_EXTENSION[ext] : undefined;
}

export function formatFileSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
