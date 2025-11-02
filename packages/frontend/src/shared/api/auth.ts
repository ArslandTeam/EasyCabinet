import { isAxiosError } from "axios";
import { axios, setBearerToken, failure } from "../lib";
import { atom, getDefaultStore } from "jotai";

export const isAuthedAtom = atom(false);
export const isLoadedAtom = atom(false);

export async function register(
  email: string,
  login: string,
  password: string,
  code: number,
) {
  try {
    await axios.post("auth/register", {
      email,
      login,
      password,
      code,
    });
  } catch (error) {
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
    return false;
  }
  return true;
}

export async function verifyEmail(email: string) {
  try {
    await axios.post("auth/verify-email", { email });
  } catch (error) {
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
    return false;
  }
  return true;
}

export async function authentication(login: string, password: string) {
  try {
    const { data } = await axios.post(
      "auth/authentication",
      { login, password },
      { withCredentials: true },
    );
    setBearerToken(data.accessToken);
    getDefaultStore().set(isAuthedAtom, true);
  } catch (error) {
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
    return false;
  }
  return true;
}

export async function resetPassword(email: string) {
  try {
    await axios.post("auth/reset-password", { email });
  } catch (error) {
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
    return false;
  }
  return true;
}

export async function changePassword(reset_token: string, password: string) {
  try {
    await axios.post("auth/change-password", { reset_token, password });
  } catch (error) {
    if (isAxiosError(error) && error.response?.data.message) {
      failure(error.response.data.message);
    } else {
      failure("Неизвестная ошибка");
    }
    return false;
  }
  return true;
}

export async function refresh() {
  try {
    const { data } = await axios.post("auth/refresh", null, {
      withCredentials: true,
    });
    setBearerToken(data.access_token);
    getDefaultStore().set(isAuthedAtom, true);
  } catch {
    // do nothing
  }
  getDefaultStore().set(isLoadedAtom, true);
}

export async function logout() {
  try {
    await axios.post("auth/logout", null, { withCredentials: true });
  } catch {
    // do nothing
  }
  getDefaultStore().set(isAuthedAtom, false);
  setBearerToken(null);
}
