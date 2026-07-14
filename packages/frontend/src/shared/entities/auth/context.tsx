import { createContext } from "react";
import type { Profile } from "../../api";

interface AuthContextType {
  isAuthed: boolean;
  isLoaded: boolean;
  profile: Profile | null;
  checkAuth: () => Promise<void>;
  fetchProfile: () => Promise<void>;
  loginSuccess: () => void;
  logoutSuccess: () => void;
}

export const AuthContext = createContext<AuthContextType | undefined>(
  undefined,
);
