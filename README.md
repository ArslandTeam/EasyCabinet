<p align="center"><img src="./packages/frontend/public/logo.png" width="200px" height="200px"></p>
<h1 align="center">EasyCabinet</h1>

Данный проект предоставляет собой базовый личный кабинет для проектов использующий [AuroraLuncher](https://github.com/AuroraTeam/AuroraLauncher)

Далее пройдите отдельно настройку и установку [frontend](https://github.com/ArslandTeam/EasyCabinet/tree/features/new-stack/packages/frontend) и [backend](https://github.com/ArslandTeam/EasyCabinet/blob/features/new-stack/packages/backend/README.md).

## Привязка к лаунчеру

Для привязки нужно только изменить конфигурацию LaunchServer на:

```hjson
auth:
{
    type: json
    authUrl: http://ДОМЕН_BACKEND_СЕРВЕРА/aurora/auth
    joinUrl: http://ДОМЕН_BACKEND_СЕРВЕРА/aurora/join
    hasJoinedUrl: http://ДОМЕН_BACKEND_СЕРВЕРА/aurora/hasJoined
    profileUrl: http://ДОМЕН_BACKEND_СЕРВЕРА/aurora/profile
    profilesUrl: http://ДОМЕН_BACKEND_СЕРВЕРА/aurora/profiles
}
injector:
{
    skinDomains: [
        "ДОМЕН_BACKEND_СЕРВЕРА"
    ]
}
```
