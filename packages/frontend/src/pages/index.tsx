import { FaWindows, FaLinux, FaApple } from "react-icons/fa";
function Index() {
  const links = [
    {
      title: "Winodws",
      icon: <FaWindows className="w-24 h-24 p-4" />,
      url: "#",
    },
    {
      title: "Linux",
      icon: <FaLinux className="w-24 h-24 p-4" />,
      url: "#",
    },
    {
      title: "MacOS",
      icon: <FaApple className="w-24 h-24 p-4" />,
      url: "#",
    },
  ];

  return (
    <div className="h-full flex flex-col items-center justify-center font-extralight">
      <h1 className="text-4xl text-center">Добро пожаловать на Project Name</h1>

      <h2 className="mt-4 text-2xl text-center">Скачать лаунчер:</h2>
      <div className="mt-4 flex flex-wrap gap-4 items-center justify-center text-center">
        {links.map(({ title, icon, url }) => (
          <a
            key={title}
            className="bg-neutral-800 hover:bg-neutral-700 rounded-md px-4 py-2"
            href={url}
            download
          >
            {icon}
            {title}
          </a>
        ))}
      </div>
    </div>
  );
}

export default Index;
