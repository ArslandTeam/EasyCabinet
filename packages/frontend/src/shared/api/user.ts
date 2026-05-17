import { atom, getDefaultStore } from "jotai";
import { axios, failure, success, setBearerToken } from "../lib";
import { isAxiosError } from "axios";

interface Profile {
  skin_url?: string;
  cape_url?: string;
  is_alex: boolean;
  sessions: string[];
}

export const profileAtom = atom<Profile | null>(null);

export function getProfile() {
  axios
    .get("users", { withCredentials: true })
    .then(({ data }) => {
      getDefaultStore().set(profileAtom, data);
      if (data.accessToken) {
        setBearerToken(data.accessToken);
      }
    })
    .catch(() => {});
}

export async function editProfile(formData: FormData) {
  try {
    await axios.put("users", formData, { withCredentials: true });
    success("Профиль успешно обновлен");
  } catch (error) {
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
  }
}
