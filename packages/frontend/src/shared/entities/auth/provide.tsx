import { useEffect, useState, type ReactNode } from "react";
import { getProfile, refresh, type Profile } from "../../api";
import { AuthContext } from "./context";

export function AuthProvider({ children }: { children: ReactNode }) {
  const [isAuthed, setIsAuthed] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);
  const [profile, setProfile] = useState<Profile | null>(null);

  const checkAuth = async () => {
    const success = await refresh();
    setIsAuthed(success);
    setIsLoaded(true);
  };

  const loginSuccess = async () => {
    setIsAuthed(true);
  };

  const logoutSuccess = async () => {
    setIsAuthed(false);
    setProfile(null);
  };

  const fetchProfile = async () => {
    const data = await getProfile();
    if (data) {
      setProfile(data);
    }
  };

  useEffect(() => {
    checkAuth();
  }, []);

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
