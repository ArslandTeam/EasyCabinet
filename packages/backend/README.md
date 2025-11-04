# Установка баз данных

Для работы вам нужно на сервер установить такие программы как:

- [MySQL](https://www.mysql.com/)

### Настройка MySQL

Нужно создать базу данных:

```sql
CREATE DATABASE `НАЗВАНИЕ_БАЗЫ`;
```

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

После первога запуска прогромма сгенерирует файл `.env`.
В этом файле будут храниться ваши настройки:
```
HOST - IP который слушает сервер
PORT - Порт на котором работает сервер
FRONTEND_URL - Адрес `frontend` части
BACKEND_URL - Внешний адрес `backend` части
JWT_SECRET - Секретный ключ для JWT. **Должен быть длиной 64 бита!**
JWT_EXPIRES_IN - Время жизни JWT токена
COOKIE_SECURE - Использовать HTTPS для куки
COOKIES_SECRET - Секретный ключ для куки. **Должен быть длиной 64 бита!**
COOKIE_EXPIRES_IN - Через сколько куки станут не действительные
DATABASE_URL - Адрес подключения к базе данных. Формат: `mysql://username:password@host:port/database`
EMAIL_FROM - Адрес отправителя почты
SMTP - Адрес почтового сервера. Формат: `smtps://username:password@host:port`
```

После настройки запустите заново

```
./backend
```

Для разработки можно запустить сервер в соответственном режиме.

```sh
RUST_LOG=debug ./backend
```
