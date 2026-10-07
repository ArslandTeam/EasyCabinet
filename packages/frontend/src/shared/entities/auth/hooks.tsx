import { use } from "react";
import { AuthContext } from "./context";

export function useAuth() {
  const context = use(AuthContext);
  if (!context) {
    throw new Error();
  }
  return context;
}
