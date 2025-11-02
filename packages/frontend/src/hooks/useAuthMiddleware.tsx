import { useEffect } from "react";
import { useNavigate } from "react-router";
import { isAuthedAtom, isLoadedAtom } from "../shared/api";
import { useAtomValue } from "jotai";

export function useAuthMiddleware() {
  const navigate = useNavigate();

  const isLoaded = useAtomValue(isLoadedAtom);
  const isAuthed = useAtomValue(isAuthedAtom);

  useEffect(() => {
    if (!isLoaded) {
      return;
    }

    if (!isAuthed) {
      navigate("/authentication", { replace: true });
    }
  });
}
