import { useEffect } from "react";
import { refresh } from "../shared/api";
import { BrowserRouter, Routes, Route } from "react-router";
import Layouts from "./layouts";
import Register from "../pages/register";
import Authentication from "../pages/authentication";
import NotFound from "../pages/not-found";
import ForgotPassword from "../pages/forgot-password";
import ChangePassword from "../pages/change-password";
import Profile from "../pages/profile";
import Index from "../pages/index";

function App() {
  useEffect(() => {
    refresh();
  }, []);
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layouts />}>
          <Route index element={<Index />} />
          <Route path="register" element={<Register />} />
          <Route path="authentication" element={<Authentication />} />
          <Route path="profile" element={<Profile />} />
          <Route path="forgot-password" element={<ForgotPassword />} />
          <Route path="change-password" element={<ChangePassword />} />
          <Route path="*" element={<NotFound />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
