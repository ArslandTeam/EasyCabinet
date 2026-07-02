import { failure, success } from "../shared/lib";
import { resetPassword } from "../shared/api";

export default function ForgotPassword() {
  const submit = async (e: React.SyntheticEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const email = formData.get("email") as string;

    if (!email) {
      return failure("Заполните все поля");
    }

    if (await resetPassword(email)) {
      success("Запрос на сброс пароля отправлен. Проверьте почту");
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Сброс пароля</h1>
        <form className="flex flex-col gap-4" onSubmit={submit}>
          <input
            type="email"
            name="email"
            placeholder="Почта"
            className="border border-neutral-700 rounded-lg p-2 bg-neutral-800 text-white"
          />
          <button className="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2">
            Сбросить пароль
          </button>
        </form>
      </div>
    </div>
  );
}
