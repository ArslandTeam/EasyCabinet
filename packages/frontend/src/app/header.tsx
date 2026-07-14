import { useNavigate } from "react-router";
import { logout } from "../shared/api";
import { useAuth } from "../shared/entities/auth/hooks";
import logo from "../assets/logo.png";
import { Link } from "react-router";

function Header() {
  const { isAuthed, logoutSuccess } = useAuth();
  const navigate = useNavigate();

  const doLogout = async () => {
    await logout();
    logoutSuccess();
    navigate("/");
  };

  return (
    <header className="flex items-center justify-between py-3">
      <Link to="/" className="flex items-center gap-4">
        <img src={logo} className="w-10" alt="Logo" />
        <span className="hidden sm:inline text-xl font-extralight">
          Easy Cabinet
        </span>
      </Link>
      {isAuthed ? (
        <nav className="flex items-center gap-4 p-4">
          <Link to="/profile">Профиль</Link>
          <Link to="#" onClick={doLogout}>
            Выход
          </Link>
        </nav>
      ) : (
        <nav className="flex items-center gap-4 p-4">
          <Link to="/authentication">Вход</Link>
          <Link to="/register">Регистрация</Link>
        </nav>
      )}
    </header>
  );
}

export default Header;
