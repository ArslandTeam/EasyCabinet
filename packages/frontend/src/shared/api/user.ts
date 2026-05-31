import { axios, failure, success } from "../lib";
import { isAxiosError } from "axios";

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
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
    return false;
  }
}
