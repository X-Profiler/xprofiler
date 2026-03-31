import { create } from 'zustand';

interface User {
  id: string;
  name: string;
  email: string;
  avatar: string;
  expPoints: number;
  currentLanguage: string;
}

interface AppState {
  user: User | null;
  setUser: (user: User | null) => void;
  updateExp: (points: number) => void;
  setCurrentLanguage: (lang: string) => void;
}

export const useStore = create<AppState>((set) => ({
  user: null,
  setUser: (user) => set({ user }),
  updateExp: (points) => set((state) => ({ 
    user: state.user ? { ...state.user, expPoints: state.user.expPoints + points } : null 
  })),
  setCurrentLanguage: (lang) => set((state) => ({
    user: state.user ? { ...state.user, currentLanguage: lang } : null
  })),
}));
