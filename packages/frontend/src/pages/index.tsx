import { FaWindows, FaLinux, FaApple } from "react-icons/fa";

const links = [
  {
    title: "Windows",
    Icon: FaWindows,
    url: "#",
  },
  {
    title: "Linux",
    Icon: FaLinux,
    url: "#",
  },
  {
    title: "MacOS",
    Icon: FaApple,
    url: "#",
  },
];

function Index() {
  return (
    <div className="h-full flex flex-col items-center justify-center font-extralight">
      <h1 className="text-4xl text-center">Добро пожаловать на Project Name</h1>

      <h2 className="mt-4 text-2xl text-center">Скачать лаунчер:</h2>
      <div className="mt-4 flex flex-wrap gap-4 items-center justify-center text-center">
        {links.map(({ title, Icon, url }) => (
          <a
            key={title}
            className="bg-neutral-800 hover:bg-neutral-700 rounded-md px-4 py-2"
            href={url}
            download
          >
            <Icon className="w-24 h-24 p-4" />
            {title}
          </a>
        ))}
      </div>
    </div>
  );
}

export default Index;
