import { IdleAnimation, SkinViewer } from "skinview3d";
import {
  accountAtom,
  editProfile,
  getAccount,
  getProfile,
  isAuthedAtom,
  isLoadedAtom,
  logout_all,
  profileAtom,
} from "../shared/api";
import { useEffect, useRef, useState } from "react";
import { useAtomValue } from "jotai";
import { useAuthMiddleware } from "../hooks/useAuthMiddleware";
import { failure } from "../shared/lib";
import { useNavigate } from "react-router";

export default function Profile() {
  useAuthMiddleware();
  const [skinType, setSkinType] = useState<boolean>(false);
  const profile = useAtomValue(profileAtom);
  const account = useAtomValue(accountAtom);
  const skinViewer = useRef<SkinViewer | null>(null);
  const skinCanvas = useRef<HTMLCanvasElement>(null);
  const isLoaded = useAtomValue(isLoadedAtom);
  const isAuthed = useAtomValue(isAuthedAtom);
  const navigate = useNavigate();

  useEffect(() => {
    if (isLoaded && isAuthed) {
      getProfile();
      getAccount();
    }
  }, [isLoaded, isAuthed]);

  const doLogoutAll = async () => {
    await logout_all();
    navigate("/");
  };
  useEffect(() => {
    if (!skinCanvas.current) return;
    const _skinViewer = new SkinViewer({
      canvas: skinCanvas.current,
      width: 300,
      height: 400,
      skin: "data:image/png;base64,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",
    });

    _skinViewer.animation = new IdleAnimation();

    _skinViewer.camera.position.x = -10;
    _skinViewer.camera.position.y = 10;
    _skinViewer.camera.position.z = 40;

    skinViewer.current = _skinViewer;
  }, []);

  useEffect(() => {
    if (!profile) return;

    if (profile.skin_url) {
      skinViewer.current?.loadSkin(profile.skin_url, {
        model: profile.is_alex ? "slim" : "default",
      });
    }

    setSkinType(!!profile.is_alex);

    if (profile.cape_url) {
      skinViewer.current?.loadCape(profile.cape_url);
    }
  }, [profile]);

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    formData.set("is_alex", String(skinType));
    editProfile(formData);
  };

  const loadSkin = (e: React.ChangeEvent<HTMLInputElement>) => {
    loadImage(e, async (img: string) => {
      await skinViewer.current?.loadSkin(img);
      setSkinType(skinViewer.current?.playerObject.skin.modelType === "slim");
    });
  };

  const loadCape = (e: React.ChangeEvent<HTMLInputElement>) => {
    loadImage(e, (img: string) =>
      skinViewer.current?.loadCape(img).catch((e) => failure(e.message)),
    );
  };

  const loadImage = (
    e: React.ChangeEvent<HTMLInputElement>,
    onload: (img: string) => void,
  ) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => onload(reader.result as string);
    reader.readAsDataURL(file);
  };

  const changeSkinType = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!skinViewer.current) return;

    skinViewer.current.playerObject.skin.modelType = e.target.checked
      ? "slim"
      : "default";
    setSkinType(e.target.checked);
  };

  const changeCapeElytra = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!skinViewer.current) return;

    if (!profile?.cape_url) {
      skinViewer.current.playerObject.backEquipment = null;
      return;
    }
    skinViewer.current.playerObject.backEquipment = e.target.checked
      ? "elytra"
      : "cape";
  };

  return (
    <div className="flex flex-col md:flex-row items-center md:items-start gap-6 mt-6 p-6 bg-neutral-800 rounded-sm">
      <div>
        <canvas ref={skinCanvas} className="rounded-sm bg-neutral-900/20" />
        <div className="flex items-center justify-center">
          <span className="text-sm font-medium text-neutral-300 mr-2">
            Плащ
          </span>
          <label className="relative inline-flex items-center cursor-pointer my-2">
            <input
              type="checkbox"
              name="isAlex"
              className="sr-only peer"
              onChange={changeCapeElytra}
            />
            <div className="w-11 h-6 peer-focus:outline-hidden peer-focus:ring-4 peer-focus:ring-blue-800 rounded-full peer bg-neutral-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-0.5 after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all border-neutral-600 peer-checked:bg-blue-600" />
          </label>
          <span className="ms-3 text-sm font-medium text-neutral-300">
            Элитры
          </span>
        </div>
      </div>
      <form
        className="max-md:flex max-md:flex-col max-md:items-center"
        onSubmit={onSubmit}
      >
        <table className="mb-6 w-full">
          <tbody>
            <tr className="max-md:flex max-md:flex-col max-md:items-center">
              <td className="font-medium w-24">Тип скина:</td>
              <td className="flex items-center">
                <span className="text-sm font-medium text-neutral-300 mr-2">
                  Default
                </span>
                <label className="relative inline-flex items-center cursor-pointer my-2">
                  <input
                    type="checkbox"
                    name="isAlex"
                    className="sr-only peer"
                    onChange={changeSkinType}
                    checked={skinType}
                  />
                  <div className="w-11 h-6 peer-focus:outline-hidden peer-focus:ring-4 peer-focus:ring-blue-800 rounded-full peer bg-neutral-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-0.5 after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all border-neutral-600 peer-checked:bg-blue-600" />
                </label>
                <span className="ms-3 text-sm font-medium text-neutral-300">
                  Slim
                </span>
              </td>
            </tr>
            <tr>
              <td className="hidden md:table-cell" />
              <td className="hidden md:table-cell">
                <small className="text-neutral-500">
                  Определяется автоматически. Переключайте, если тип скина
                  определился с ошибкой.
                </small>
              </td>
              <td className="md:hidden text-center" colSpan={2}>
                <small className="text-neutral-500">
                  Определяется автоматически. Переключайте, если тип скина
                  определился с ошибкой.
                </small>
              </td>
            </tr>
          </tbody>
        </table>
        <div className="max-md:flex max-md:flex-col max-md:items-center max-md:justify-center">
          <input
            type="file"
            name="skin"
            id="skin"
            hidden
            accept="image/png"
            onChange={loadSkin}
          />
          <label
            htmlFor="skin"
            className="inline-block px-4 py-2 border border-neutral-600 rounded-lg hover:bg-neutral-700 transition-colors cursor-pointer md:mr-3"
          >
            Загрузить скин
          </label>
          <input
            type="file"
            name="cape"
            id="cape"
            hidden
            accept="image/png"
            onChange={loadCape}
          />
          <label
            htmlFor="cape"
            className="inline-block px-4 py-2 border border-neutral-600 rounded-lg hover:bg-neutral-700 transition-colors cursor-pointer mt-2 md:mr-3"
          >
            Загрузить плащ
          </label>
        </div>
        <br />
        <button className="px-4 py-2 mt-6 border bg-blue-600 border-blue-700 hover:bg-blue-500 hover:border-blue-600 transition-colors rounded-lg save-button md:mr-3">
          Сохранить изменения
        </button>
      </form>

      {/*TODO пределать разметку под нормальную*/}
      <div className="space-y-3">
        {account?.sessions.map((session, index) => (
          <div
            key={session.id || index}
            className="p-4 border border-zinc-700 bg-zinc-800 rounded-lg"
          >
            <p className="text-sm text-zinc-300">
              <span className="font-semibold">Устройство:</span>{" "}
              {session.user_agent}
            </p>
            <p className="text-xs text-zinc-500">
              <span className="font-semibold">Дата входа:</span>{" "}
              {new Date(session.iat).toLocaleString()}
            </p>
          </div>
        ))}
      </div>

      <a href="#" onClick={doLogoutAll}>
        Выход
      </a>
    </div>
  );
}
