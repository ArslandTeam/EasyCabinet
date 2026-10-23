# Установка баз данных

Для работы вам нужно на сервер установить такие программы как:

- [PostgreSQL](https://www.postgresql.org/)
- [Redis](https://redis.io) Нужен для хранения кеша ввиде токенов

Опциональные программы:
- S3 Нужен для хранения текстур скинов и плащей игроков. Используйте Amazon или подобные ему хранилища (Minio как из вариантов)

### Настройка PostgreSQL

Cоздайте базу данных и выдайте права на запись пользователю:

```sql
CREATE USER `пользователь` WITH PASSWORD 'пароль';
CREATE DATABASE `название базы данных` OWNER `пользовтаель`;
```

# Запуск

[Скачайте бинарный файл с релиза](https://github.com/ArslandTeam/EasyCabinet/releases/latest)

Выдайте права

```
chmod +x ./backend_linux_x64
```

Запустите

```
./backend_linux_x64
```

Пример комманды для генерации секрета для JWT токенов и Cookie: 
```
openssl rand -base64 64
```

После первого запуска программа сгенерирует файл `.env`.
В этом файле будут храниться ваши настройки:
```
HOST - IP который слушает сервер
PORT - Порт на котором работает сервер
BACKEND_URL - Внешний андресс backend приложения
FRONTEND_URL - Адрес `frontend` части
JWT_SECRET - Секретный ключ для JWT. Должен быть длиной 64 символа!
JWT_EXPIRES_IN - Время жизни JWT токена
COOKIE_SECURE - Использовать HTTPS для куки
COOKIES_SECRET - Секретный ключ для куки. Должен быть длиной 64 символа!
COOKIE_DOMAIN - на каком домене должны располаться cookies
COOKIE_PATH - Путь до cookies (удобно при сценарии есть backend расположен на одном домене, с другими API сервисами)
COOKIE_EXPIRES_IN - Через сколько куки станут не действительные. *В секундах
REDIS_URL - Адрес подключения к Redis. Формат: `redis://host:port`
STORAGE_TEXTURES_TYPE - Тип хранилища текстур для скинов и плащей. Может быть `s3` или `local`
AWS_REGION - Регион сервера Amazon.
AWS_ENDPOINT_URL - Адресс подключения к S3 хранилищу
AWS_ACCESS_KEY_ID - Логин для подключения к S3 хранилищу
AWS_SECRET_ACCESS_KEY - Пароль от пользователя S3 хранилища
BUCKET_NAME - Имя контейнера для хранения текстур
DATABASE_URL - Адрес подключения к базе данных. Формат: `postgresql://username:password@host:port/database`
MAX_CONNECTION="30" - количество соединений с базой данных
CONNECT_TIMEOUT="10" - время ожидания покдлючения к базе данных
SQL_LOG - логирование базы данных
CLIENT_ID_DISCORD - ID Discord приложения
CLIENT_SECRET_DISCORD - Секретный ключ Discord приложения
EMAIL_FROM - Адрес отправителя почты
SMTP - Адрес почтового сервера. Формат: `smtps://username:password@host:port`
```

После настройки запустите заново

```
./backend_linux_x64
```

# Разработка

Установите язык программирования [Rust](https://rust-lang.org/learn/get-started/)

Запукс программы в dev режиме с логированием
```
RUST_LOG=debug cargo run dev
```
