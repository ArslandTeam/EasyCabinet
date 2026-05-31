import { useEffect } from "react";
import { useNavigate } from "react-router";
import { useAuth } from "../hooks";
export function useAuthMiddleware() {
  const navigate = useNavigate();
  const { isAuthed, isLoaded } = useAuth();

  useEffect(() => {
    if (!isLoaded) {
      return;
    }
    if (!isAuthed) {
      navigate("/authentication", { replace: true });
    }
  }, [isAuthed, isLoaded, navigate]);
}
