# Установка баз данных

Для работы вам нужно на сервер установить такие программы как:

- [MySQL](https://www.mysql.com/)

Опциональные программы:

- [Redis](https://redis.io) Нужен для хранения кеша ввиде токенов
- S3 Нужен для хранения текстур скинов и плащей игроков. Используйте Amazon или подобные ему хранилища (Minio как из вариантов)

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

После первого запуска программа сгенерирует файл `.env`.
В этом файле будут храниться ваши настройки:
- HOST - IP который слушает сервер
- PORT - Порт на котором работает сервер
- BACKEND_URL - Внешний андресс backend приложения
- FRONTEND_URL - Адрес `frontend` части
- JWT_SECRET - Секретный ключ для JWT. **Должен быть длиной 64 бита!**
- JWT_EXPIRES_IN - Время жизни JWT токена
- COOKIE_SECURE - Использовать HTTPS для куки
- COOKIES_SECRET - Секретный ключ для куки. **Должен быть длиной 64 бита!**
- COOKIE_EXPIRES_IN - Через сколько куки станут не действительные
- CACHE - Тип кеша. Может быть `redis` или `local`
- REDIS_URL - Адрес подключения к Redis. Формат: `redis://host:port`
- STORAGE_TEXTURES_TYPE - Тип хранилища текстур для скинов и плащей. Может быть `s3` или `local`
- AWS_REGION - Регион сервера Amazon.
- AWS_ENDPOINT_URL - Адресс подключения к S3 хранилищу
- AWS_ACCESS_KEY_ID - Логин для подключения к S3 хранилищу
- AWS_SECRET_ACCESS_KEY - Пароль от пользователя S3 хранилища
- BUCKET_NAME - Имя контейнера для хранения текстур
- DATABASE_URL - Адрес подключения к базе данных. Формат: `mysql://username:password@host:port/database`
- EMAIL_FROM - Адрес отправителя почты
- SMTP - Адрес почтового сервера. Формат: `smtps://username:password@host:port`

После настройки запустите заново

```
./backend
```

# Разработка

Установите язык программирования [Rust](https://rust-lang.org/learn/get-started/)

Запукс программы в dev режиме с логированием
```
RUST_LOG=debug cargo run dev
```
