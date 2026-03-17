import { create } from 'zustand'

export type UserRole = 'operator' | 'approver' | 'admin' | 'analyst'

type AuthState = {
  token: string | null
  username: string | null
  role: UserRole
  setAuth: (next: { token: string; username: string; role: UserRole }) => void
  logout: () => void
}

const KEY = 'ops_auth'

function loadInitial(): Pick<AuthState, 'token' | 'username' | 'role'> {
  const raw = localStorage.getItem(KEY)
  if (!raw) return { token: null, username: null, role: 'operator' }
  try {
    const v = JSON.parse(raw) as any
    return {
      token: typeof v?.token === 'string' ? v.token : null,
      username: typeof v?.username === 'string' ? v.username : null,
      role: (v?.role as UserRole) ?? 'operator',
    }
  } catch {
    return { token: null, username: null, role: 'operator' }
  }
}

export const useAuthStore = create<AuthState>((set, get) => ({
  ...loadInitial(),
  setAuth: (next) => {
    localStorage.setItem(KEY, JSON.stringify(next))
    set({ token: next.token, username: next.username, role: next.role })
  },
  logout: () => {
    localStorage.removeItem(KEY)
    set({ token: null, username: null, role: 'operator' })
  },
}))

export function actorHeaderValue(state: Pick<AuthState, 'username' | 'role'>): string {
  const u = state.username ? state.username.trim() : ''
  const role = state.role
  return u ? `${role}:${u}` : role
}

