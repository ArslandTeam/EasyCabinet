# Установка баз данных

Для работы вам нужно на сервер установить такие программы как:

- [MySQL](https://www.mysql.com/)

### Настройка MySQL

Нужно создать базу данных:

```sql
CREATE DATABASE `НАЗВАНИЕ_БАЗЫ`;
```

# Настройка

Переименуйте файл `.env.example` в `.env`.

В этом файле будут храниться ваши настройки:

- `HOST` - IP который слушает сервер
- `PORT` - Порт на котором работает сервер
- `FRONTEND_URL` - Адрес `frontend` части
- `BACKEND_URL` - Внешний адрес `backend` части
- `JWT_SECRET` - Секретный ключ для JWT
- `JWT_EXPIRES_IN` - Время жизни JWT токена
- `COOKIE_SECURE` - Использовать HTTPS для куки
- `COOKIE_EXPIRES_IN` - Через сколько куки станут не действительные
- `DATABASE_URL` - Адрес подключения к базе данных. Формат: `mysql://username:password@host:port/database`
- `EMAIL_FROM` - Адрес отправителя почты
- `SMTP` - Адрес почтового сервера. Формат: `smtps://username:password@host:port`

# Запуск

[Скачайте бинарный файл с релиза](https://github.com/ArslandTeam/EasyCabinet/releases)

Выдайте права

```
chmod +x ./backend
```

Запустите

```
./backend
```

Для разработки можно запустить сервер в соответственном режиме (требуется наличие Rust).

```sh
RUST_LOG=debug cargo run dev
```
