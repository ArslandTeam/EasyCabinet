import { axios, handleError } from "../lib";


const discordParams = new URLSearchParams({
  client_id: import.meta.env.VITE_DISCORD_CLIENT_ID,
  redirect_uri: `${import.meta.env.VITE_API_URL}/auth/discord`,
  response_type: "code",
  scope: "identify email",
});

export const DISCORD_AUTH_URL = `https://discord.com/oauth2/authorize?${discordParams.toString().replace(/\+/g, '%20')}`;

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
    return false
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
    await axios.post("auth/logout", null);
  } catch {
    // do nothing
  }
}

export async function logout_all() {
  try {
    await axios.post("auth/logout_all", null);
  } catch {
    // do nothing
  }
}
