import { axios, handleError, success } from "../lib";

export interface Profile {
  login: string;
  textures: Textures;
  sessions: string[];
}

interface Textures {
  is_alex: boolean;
  skin_url?: string;
  cape_url?: string;
}

export async function getProfile(): Promise<Profile | null> {
  try {
    const { data } = await axios.get<Profile>("users", {
      withCredentials: true,
    });
    return data;
  } catch {
    return null;
  }
}

export async function editProfile(formData: FormData): Promise<boolean> {
  try {
    await axios.put("users", formData, { withCredentials: true });
    success("Профиль успешно обновлен");
    return true;
  } catch (error) {
    handleError(error);
    return false;
  }
}

export async function editPassword(password: string) {
  try {
    await axios.post("users/change-password", { password });
    success("Пароль успешно обновлен");
    return true;
  } catch (error) {
    handleError(error);
    return false;
  }
}

export async function editEmail(email: string, code: number) {
  try {
    await axios.post("users/change-email", { email, code });
    success("Почта успешна обновлена");
    return true;
  } catch (error) {
    handleError(error);
    return false;
  }
}
