import { failure, success } from "../shared/lib";
import { register, verifyEmail } from "../shared/api";
import { useNavigate } from "react-router";
import { Link } from "react-router";

export default function Register() {
  const navigate = useNavigate();

  const verifyEmailSubmit = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const form = e.currentTarget.closest("form");
    if (!form) return;
    const formData = new FormData(form);

    const email = formData.get("email") as string;

    if (!email) {
      return failure("Заполните поле почты");
    }

    if (await verifyEmail(email)) {
      success("Код потдверждения почты отправлен. Проверьте почту");
    }
  };

  const submit = async (e: React.SyntheticEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);

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

    if (await register(email, login, password, code)) {
      success("Регистрация прошла успешно");
      navigate("/authentication");
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Регистрация</h1>
        <form className="flex flex-col gap-4" onSubmit={submit}>
          <input
            type="text"
            name="login"
            placeholder="Логин"
            maxLength={16}
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <input
            type="email"
            name="email"
            placeholder="Почта"
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <input
            type="password"
            name="password"
            placeholder="Пароль"
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <input
            type="password"
            name="password_forgot"
            placeholder="Повторите пароль"
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <div className="grid grid-cols-2 gap-3">
            <input
              type="number"
              name="code"
              placeholder="Код"
              min="100000"
              max="999999"
              className="[appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
            />
            <button
              type="button"
              onClick={verifyEmailSubmit}
              className="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2"
            >
              Получить код
            </button>
          </div>
          <button
            type="submit"
            className="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2"
          >
            Зарегистрироваться
          </button>
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
