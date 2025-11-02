import { Outlet } from "react-router";
import Header from "./header";

function Layouts() {
  return (
    <>
      <Header />
      <main className="grid min-h-[100vh]">
        <Outlet />
      </main>
    </>
  );
}

export default Layouts;
