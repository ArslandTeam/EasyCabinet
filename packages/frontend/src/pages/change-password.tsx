import { failure, success } from "../shared/lib";
import { changePassword } from "../shared/api";
import { useNavigate } from "react-router";
import { SubmitButton } from "../shared/ui/SubmitButton";

export default function ChangePassword() {
  const navigate = useNavigate();

  const action = async (formData: FormData) => {

    const searchParams = new URLSearchParams(window.location.search);
    const resetToken = searchParams.get("resetToken");

    if (!resetToken) {
      return failure("Отсутствует токен сброса пароля");
    }

    const password = formData.get("password") as string;
    const password_forgot = formData.get("password_forgot") as string;

    if (!password || !password_forgot) {
      return failure("Заполните все поля");
    }

    if (password !== password_forgot) {
      return failure("Пароли не совпадают");
    }

    try {
      if (await changePassword(resetToken, password)) {
        success("Пароль успешно изменен");
        navigate("/authentication");
      }
    } catch {
      // do nothing
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Смена пароля</h1>
        <form className="flex flex-col gap-4" action={action}>
          <input
            type="password"
            name="password"
            placeholder="Введите новый пароль"
            required
          />
          <input
            type="password"
            name="password_forgot"
            placeholder="Повторите пароль"
            required
          />
          <SubmitButton pendingText="Сохранение..." style="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2">Сохранить</SubmitButton>
        </form>
      </div>
    </div>
  );
}
