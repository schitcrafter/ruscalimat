import { localStorageStore } from '@skeletonlabs/skeleton';
import type { Writable } from 'svelte/store';

export const authHeader: Writable<string> = localStorageStore('authHeader', "");
