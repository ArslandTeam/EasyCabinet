import { IdleAnimation, SkinViewer } from "skinview3d";
import { editProfile, logout_all, revokeSession } from "../shared/api";
import { useEffect, useRef, useState } from "react";
import { useAuthMiddleware } from "../shared/entities/auth/model/useAuthMiddleware";
import { failure, success } from "../shared/lib";
import { useNavigate } from "react-router";
import { useAuth } from "../shared/entities/auth";
import defaultSkin from "../assets/steve.png";

export default function Profile() {
  useAuthMiddleware();
  const { isAuthed, isLoaded, profile, fetchProfile, logoutSuccess } = useAuth();
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

  const doRevokeSession = async (sessionId: string) => {
    const target = profile?.sessions.find((session) => session.id === sessionId);

    if (await revokeSession(sessionId)) {
      if (target?.current) {
        logoutSuccess();
        navigate("/");
        return;
      }

      fetchProfile();
      success("Сессия успешно завершена");
    }
  };


  useEffect(() => {
    if (!skinCanvas.current) return;
    const skin = new SkinViewer({
      canvas: skinCanvas.current,
      width: 300,
      height: 400,
      skin: defaultSkin,
      model: "default",
    });

    skin.animation = new IdleAnimation();

    skin.camera.position.x = -10;
    skin.camera.position.y = 10;
    skin.camera.position.z = 40;

    skinViewer.current = skin;

    return () => {
      skin.dispose();
    };
  }, []);

  useEffect(() => {
    if (!profile || !skinViewer.current) return;

    if (profile.textures.skin_url) {
      skinViewer.current.loadSkin(profile.textures.skin_url, {
        model: profile.textures.is_alex ? "slim" : "default",
      });
      setSkinType(!!profile.textures.is_alex);
    } else {
      skinViewer.current.loadSkin(defaultSkin, {
        model: "default",
      });
      setSkinType(false);
    }

    if (profile.textures.cape_url) {
      skinViewer.current.loadCape(profile.textures.cape_url);
    } else {
      skinViewer.current.playerObject.backEquipment = null;
    }
  }, [profile]);

  const onSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
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

  // const changeCapeElytra = (e: React.ChangeEvent<HTMLInputElement>) => {
  //   if (!skinViewer.current) return;

  //   if (!profile?.textures.cape_url) {
  //     skinViewer.current.playerObject.backEquipment = null;
  //     return;
  //   }
  //   skinViewer.current.playerObject.backEquipment = e.target.checked
  //     ? "elytra"
  //     : "cape";
  // };

  return (
    <div className="flex flex-col md:flex-row items-center md:items-start gap-6 mt-6 p-6 bg-neutral-800 rounded-sm">

      <div className="flex flex-col items-center">
        {profile?.login && <span className="border-2 border-stone-700 p-1 mb-1 rounded-sm">{profile.login}</span>}
        <canvas ref={skinCanvas} className="rounded-sm bg-neutral-900/20" />
        <div className="flex items-center justify-center">
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
            <div className="w-11 h-6 peer-focus:outline-hidden rounded-full peer bg-neutral-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:inset-s-0.5 after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all border-neutral-600 peer-checked:bg-primary-hover" />
          </label>
          <span className="ms-3 text-sm font-medium text-neutral-300">
            Slim
          </span>
        </div>
      </div>

      <form
        className="max-md:flex max-md:flex-col max-md:items-center"
        onSubmit={onSubmit}
      >
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
        <button className="px-4 py-2 mt-4 bg-primary hover:bg-primary-hover transition-colors rounded-lg save-button md:mr-3">
          Сохранить изменения
        </button>
      </form>

      <div className="flex flex-col items-center p-3 rounded-sm max-w-87.5 border border-stone-700">
        <span className="font-bold">Сессии</span>
        {profile?.sessions.map((session, index) => (
          <div key={index} className="flex flex-col text-sm mt-2 bg-neutral-900/20 text-neutral-300 p-2">
            <div className="flex justify-between items-center gap-2">
              <p>{session.user_agent}</p>
              <button
                className="w-8 h-8 text-2xl"
                onClick={() => doRevokeSession(session.id)}
              >
                &times;
                </button>
            </div>
            {session.current && <span className="mt-2 text-sm rounded-sm bg-primary self-start px-2 py-0.5">Текущая</span>}
          </div>
        ))}

        <button
          type="button"
          onClick={doLogoutAll}
          className="mt-2 border border-neutral-600 rounded-lg hover:bg-neutral-700 transition-colors p-2"
        >
          Выход
        </button>
      </div>
    </div>
  );
}
