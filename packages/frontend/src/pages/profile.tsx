import { IdleAnimation, SkinViewer } from "skinview3d";
import { editProfile, logout_all } from "../shared/api";
import { useEffect, useRef, useState } from "react";
import { useAuthMiddleware } from "../entities/auth/model/useAuthMiddleware";
import { failure } from "../shared/lib";
import { useNavigate } from "react-router";
import { useAuth } from "../entities/auth";
import defaultSkin from "../assets/steve.png";

export default function Profile() {
  useAuthMiddleware();
  const { isAuthed, isLoaded, profile, fetchProfile, logoutSuccess } =
    useAuth();
  const [skinType, setSkinType] = useState<boolean>(false);
  const skinViewer = useRef<SkinViewer | null>(null);
  const skinCanvas = useRef<HTMLCanvasElement>(null);
  const navigate = useNavigate();

  useEffect(() => {
    if (isLoaded && isAuthed && !profile) {
      fetchProfile();
    }
  }, [isLoaded, isAuthed, profile, fetchProfile]);

  const doLogoutAll = async () => {
    await logout_all();
    logoutSuccess();
    navigate("/");
  };
  useEffect(() => {
    if (!skinCanvas.current) return;
    const skin = new SkinViewer({
      canvas: skinCanvas.current,
      width: 300,
      height: 400,
      skin: defaultSkin,
    });

    skin.animation = new IdleAnimation();

    skin.camera.position.x = -10;
    skin.camera.position.y = 10;
    skin.camera.position.z = 40;

    skinViewer.current = skin;
  }, []);

  useEffect(() => {
    if (!profile) return;

    if (profile.textures.skin_url) {
      skinViewer.current?.loadSkin(profile.textures.skin_url, {
        model: profile.textures.is_alex ? "slim" : "default",
      });
    }

    setSkinType(!!profile.textures.is_alex);

    if (profile.textures.cape_url) {
      skinViewer.current?.loadCape(profile.textures.cape_url);
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

    if (!profile?.textures.cape_url) {
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

      <div className="flex flex-col items-center bg-neutral-700 p-3 rounded-sm max-w-[350px] w-full">
        <span className="font-bold">Сессии</span>

        {profile?.sessions.map((userAgent, index) => (
          <p key={index} className="mt-2">
            {userAgent}
          </p>
        ))}

        <button
          type="button"
          onClick={doLogoutAll}
          className="mt-2 bg-blue-600 border-blue-700 hover:bg-blue-500 hover:border-blue-600 transition-colors rounded-lg p-2"
        >
          Выход
        </button>
      </div>
    </div>
  );
}
