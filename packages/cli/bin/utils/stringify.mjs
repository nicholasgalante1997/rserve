import { optional } from "./option.mjs";

export function safeStringify(o) {
  if (typeof o === "object") {
    const { data, error } = optional(() => JSON.stringify(o, null, 2));
    if (!data && error) {
      return o;
    }
    return data;
  }
  return o;
}
