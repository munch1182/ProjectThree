import { get, post } from "./request";

export const openWithSystemBrower = (addr: string) => get("/cmd/1/".concat(addr))