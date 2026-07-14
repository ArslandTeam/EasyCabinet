import _axios, { isAxiosError } from "axios";
import { failure } from "./notiflix";

export const axios = _axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,
});

export const handleError = (e: unknown): never => {
  if (isAxiosError(e) && e.response?.data.message) {
    throw failure(e.response.data.message);
  } else {
    throw failure("Неизвестная ошибка");
  }
};
