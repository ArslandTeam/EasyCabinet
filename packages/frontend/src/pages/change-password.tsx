import { failure, success } from "../shared/lib";
import { changePassword } from "../shared/api";
import { useNavigate } from "react-router";

export default function ChangePassword() {
  const navigate = useNavigate();

  const submit = async (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault();

    const searchParams = new URLSearchParams(window.location.search);
    const resetToken = searchParams.get("resetToken");

    if (!resetToken) {
      return failure("Отсутствует токен сброса пароля");
    }

    const formData = new FormData(e.currentTarget);

    const password = formData.get("password") as string;
    const password_forgot = formData.get("password_forgot") as string;

    if (!password || !password_forgot) {
      return failure("Заполните все поля");
    }

    if (password !== password_forgot) {
      return failure("Пароли не совпадают");
    }

    if (await changePassword(resetToken, password)) {
      success("Пароль успешно изменен");
      navigate("/authentication");
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Смена пароля</h1>
        <form className="flex flex-col gap-4" onSubmit={submit}>
          <input
            type="password"
            name="password"
            placeholder="Введите новый пароль"
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <input
            type="password"
            name="password_forgot"
            placeholder="Повторите пароль"
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <button className="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2">
            Сохранить
          </button>
        </form>
      </div>
    </div>
  );
}
