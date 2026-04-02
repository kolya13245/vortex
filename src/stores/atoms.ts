import { atom } from "jotai";
import type { CoreType, ConnectionStatus } from "@/types/core";

export const coreTypeAtom = atom<CoreType>("mihomo");
export const connectionStatusAtom = atom<ConnectionStatus>("disconnected");
export const activePageAtom = atom<string>("dashboard");
