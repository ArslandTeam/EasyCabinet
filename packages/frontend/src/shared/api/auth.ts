import { isAxiosError } from "axios";
import { axios, failure } from "../lib";

const handleError = (e: unknown): never => {
  if (isAxiosError(e) && e.response?.data.message) {
    throw failure(e.response.data.message);
  } else {
    throw failure("Неизвестная ошибка");
  }
};

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
    handleError(error);
  }
  return true;
}

export async function verifyEmail(email: string) {
  try {
    await axios.post("auth/verify-email", { email });
  } catch (error) {
    handleError(error);
    return false;
  }
  return true;
}

export async function authentication(login: string, password: string) {
  try {
    await axios.post("auth/login", { login, password });
  } catch (error) {
    handleError(error);
    return false;
  }
  return true;
}

export async function resetPassword(email: string) {
  try {
    await axios.post("auth/reset-password", { email });
  } catch (error) {
    handleError(error);
    return false;
  }
  return true;
}

export async function changePassword(reset_token: string, password: string) {
  try {
    await axios.post("auth/change-password", { reset_token, password });
  } catch (error) {
    handleError(error);
    return false;
  }
  return true;
}

export async function refresh() {
  try {
    await axios.post("auth/refresh", null, {
      withCredentials: true,
    });
    return true;
  } catch {
    return false;
  }
}

export async function logout() {
  try {
    await axios.post("auth/logout", null, { withCredentials: true });
  } catch {
    // do nothing
  }
}

export async function logout_all() {
  try {
    await axios.post("auth/logout_all", null, { withCredentials: true });
  } catch {
    // do nothing
  }
}
