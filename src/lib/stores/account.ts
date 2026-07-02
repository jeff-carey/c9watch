import { writable } from 'svelte/store';
import type { Account } from '$lib/types';
import { getAccount } from '$lib/api';

/**
 * The globally logged-in Claude account. `null` until first fetched, or when
 * logged out / unreadable. Login is machine-global in Claude Code, so this is
 * shared across all sessions rather than being a per-session field.
 */
export const account = writable<Account | null>(null);

/** Re-read the account and update the store. Cheap (a single local file read
 *  on desktop); safe to call on every session refresh so it tracks /login. */
export async function refreshAccount(): Promise<void> {
	try {
		account.set(await getAccount());
	} catch (e) {
		console.error('[account] failed to fetch:', e);
	}
}
