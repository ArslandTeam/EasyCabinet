import { Outlet } from "react-router";
import Header from "./header";

function Layouts() {
  return (
    <>
      <Header />
      <main className="grid min-h-[calc(100vh-80px)]">
        <Outlet />
      </main>
    </>
  );
}

export default Layouts;
