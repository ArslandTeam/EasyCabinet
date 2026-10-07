import { useNavigate } from "react-router";
import { failure } from "../shared/lib";
import { authentication } from "../shared/api";
import { useAuth } from "../shared/entities/auth";
import { Link } from "react-router";
import { SubmitButton } from "../shared/ui/SubmitButton";

function Authentication() {
  const navigate = useNavigate();
  const { loginSuccess } = useAuth();

  const action = async (formData: FormData) => {
    const login = formData.get("login") as string;
    const password = formData.get("password") as string;

    if (!login || !password) {
      return failure("Заполните все поля");
    }

    try {
      if (await authentication(login, password)) {
        loginSuccess();
        navigate("/profile");
      }
    } catch {
      // do nothing
    }
  };
  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Вход</h1>
        <form className="flex flex-col gap-4" action={action}>
          <input
            type="text"
            name="login"
            placeholder="Логин"
            required
          />
          <input
            type="password"
            name="password"
            placeholder="Пароль"
            required
          />
          <SubmitButton pendingText="Вход..." style="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2">Войти</SubmitButton>
        </form>
        <div className="flex flex-col gap-2 mt-4 text-center text-sm">
          <span>
            Нет аккаунта?
            <Link
              to="/register"
              className="text-blue-500 ml-1 hover:text-blue-600"
            >
              Зарегистрироваться
            </Link>
          </span>
          <span>
            <Link
              to="/forgot-password"
              className="text-blue-500 ml-1 hover:text-blue-600"
            >
              Забыли пароль?
            </Link>
          </span>
        </div>
      </div>
    </div>
  );
}

export default Authentication;
