import { useCallback, useEffect, useState, type ReactNode } from "react";
import { getProfile, refresh, type Profile } from "../../api";
import { AuthContext } from "./context";

export function AuthProvider({ children }: { children: ReactNode }) {
  const [isAuthed, setIsAuthed] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);
  const [profile, setProfile] = useState<Profile | null>(null);

  const checkAuth = useCallback(async () => {

    if (!localStorage.getItem("refresh_token")) {
      setIsAuthed(false);
      setIsLoaded(true);
      return;
    }

    const success = await refresh();
    setIsAuthed(success);

    if (!success) {
      localStorage.removeItem("refresh_token");
    }

    setIsLoaded(true);
  }, []);

  const loginSuccess = useCallback(async () => {
    setIsAuthed(true);
  }, []);

  const logoutSuccess = useCallback(async () => {
    setIsAuthed(false);
    setProfile(null);
  }, []);

  const fetchProfile = useCallback(async () => {
    const data = await getProfile();
    if (data) {
      setProfile(data);
    }
  }, []);

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  return (
    <AuthContext.Provider
      value={{
        isAuthed,
        isLoaded,
        profile,
        checkAuth,
        fetchProfile,
        loginSuccess,
        logoutSuccess,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}
