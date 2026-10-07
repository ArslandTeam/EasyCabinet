import { failure, success } from "../shared/lib";
import { register, verifyEmail } from "../shared/api";
import { useNavigate } from "react-router";
import { Link } from "react-router";
import { SubmitButton } from "../shared/ui/SubmitButton";

export default function Register() {
  const navigate = useNavigate();

  // TODO переписать
  const verifyEmailSubmit = async (e: React.MouseEvent<HTMLButtonElement>) => {
    const form = e.currentTarget.closest("form");
    if (!form) return;

    const emailInput = form.querySelector('input[name="email"]') as HTMLInputElement;
    if (!emailInput) return;

    if (!emailInput.reportValidity()) {
      return;
    }

    const email = emailInput.value;

    try {
      if (await verifyEmail(email)) {
        success("Код подтверждения почты отправлен. Проверьте почту");
      }
    } catch {
      // do nothing
    }
  };


  const action = async (formData: FormData) => {

    const login = formData.get("login") as string;
    const email = formData.get("email") as string;
    const password = formData.get("password") as string;
    const password_forgot = formData.get("password_forgot") as string;
    const code = Number(formData.get("code"));

    if (!login || !email || !password || !password_forgot || !code) {
      return failure("Заполните все поля");
    }

    if (password !== password_forgot) {
      return failure("Пароли не совпадают");
    }

    try {
      if (await register(email, login, password, code)) {
        success("Регистрация прошла успешно");
        navigate("/authentication");
      }
    } catch {
     // do nothing
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Регистрация</h1>
        <form className="flex flex-col gap-4" action={action}>
          <input
            type="text"
            name="login"
            placeholder="Логин"
            maxLength={16}
            required
          />
          <input
            type="email"
            name="email"
            placeholder="Почта"
            required
          />
          <input
            type="password"
            name="password"
            placeholder="Пароль"
            required
          />
          <input
            type="password"
            name="password_forgot"
            placeholder="Повторите пароль"
            required
          />
          <div className="grid grid-cols-2 gap-3">
            <input
              type="number"
              name="code"
              placeholder="Код"
              min="100000"
              max="999999"
              required
              className="[appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0"
            />
            <button
              type="button"
              onClick={verifyEmailSubmit}
              className="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2"
            >
              Получить код
            </button>
          </div>
          <SubmitButton pendingText="Регистрация..." style="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2">Регистрация</SubmitButton>
        </form>
        <div className="mt-4 text-center text-sm">
          Уже есть аккаунт?
          <Link
            to="/authentication"
            className="text-blue-500 ml-1 hover:text-blue-600"
          >
            Войти
          </Link>
        </div>
      </div>
    </div>
  );
}
