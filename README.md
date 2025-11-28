<p align="center"><img src="./packages/frontend/public/logo.png" width="200px" height="200px"></p>
<h1 align="center">EasyCabinet</h1>

Данный проект предоставляет собой базовый личный кабинет для проектов использующий [AuroraLuncher](https://github.com/AuroraTeam/AuroraLauncher) и [GML](https://gml.recloud.tech/)

Далее пройдите отдельно настройку и установку [frontend](https://github.com/ArslandTeam/EasyCabinet/tree/features/new-stack/packages/frontend) и [backend](https://github.com/ArslandTeam/EasyCabinet/blob/features/new-stack/packages/backend/README.md).

## Привязка к лаунчеру

### GML
В админ панеле в пункте авторизация выберите вариант EasyCabinet и в поле укажите ссылку на backend сайта


### AuroraLauncher
Для привязки нужно только изменить конфигурацию LaunchServer на:

```hjson
auth:
{
    type: json
    authUrl: https://ДОМЕН_BACKEND_СЕРВЕРА/aurora/auth
    joinUrl: https://ДОМЕН_BACKEND_СЕРВЕРА/aurora/join
    hasJoinedUrl: https://ДОМЕН_BACKEND_СЕРВЕРА/aurora/hasJoined
    profileUrl: https://ДОМЕН_BACKEND_СЕРВЕРА/aurora/profile
    profilesUrl: https://ДОМЕН_BACKEND_СЕРВЕРА/aurora/profiles
}
injector:
{
    skinDomains: [
        "ДОМЕН_BACKEND_СЕРВЕРА"
    ]
}
```
