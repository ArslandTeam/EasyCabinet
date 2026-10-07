import { failure, success } from "../shared/lib";
import { resetPassword } from "../shared/api";
import { SubmitButton } from "../shared/ui/SubmitButton";

export default function ForgotPassword() {
  const action = async (formData: FormData) => {

    const email = formData.get("email") as string;

    if (!email) {
      return failure("Заполните все поля");
    }

    try {
      if (await resetPassword(email)) {
        success("Запрос на сброс пароля отправлен. Проверьте почту");
      }
    } catch {
      // do nothing
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <div className="bg-neutral-800 p-8 rounded-lg max-[350px]:w-full w-87.5">
        <h1 className="text-3xl mb-4 text-center">Сброс пароля</h1>
        <form className="flex flex-col gap-4" action={action}>
          <input
            type="email"
            name="email"
            placeholder="Почта"
            required
          />
          <SubmitButton pendingText="Сбросить пароль..." style="bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg p-2">Сбросить пароль</SubmitButton>
        </form>
      </div>
    </div>
  );
}
