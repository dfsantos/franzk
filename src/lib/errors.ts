import type { AppErrorPayload } from "./types";

export function errorMessage(error: unknown): string {
  const payload = error as Partial<AppErrorPayload> | undefined;
  if (payload && typeof payload.message === "string") {
    return payload.message;
  }
  return String(error);
}
