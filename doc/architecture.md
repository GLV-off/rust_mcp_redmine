# Архитектура

## Обзор

```
┌──────────────────────────────────────────────────────────────┐
│                   MCP-клиент (AI-ассистент)                  │
│                   JSON-RPC over stdin/stdout                 │
└─────────────────────────┬────────────────────────────────────┘
                          │
┌─────────────────────────▼────────────────────────────────────┐
│                   redmine-mcp (binary)                       │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐   │
│  │  RedmineServer (tools.rs)                             │   │
│  │  #[tool] redmine_list_issues                          │   │
│  │  #[tool] redmine_get_issue                            │   │
│  │  #[tool] redmine_create_issue                         │   │
│  │  #[tool] redmine_update_issue                         │   │
│  │  #[tool] redmine_delete_issue                         │   │
│  │  #[tool] redmine_list_projects                        │   │
│  │  #[tool] redmine_get_project                          │   │
│  │  #[tool] redmine_list_users                           │   │
│  │  #[tool] redmine_get_user                             │   │
│  │  #[tool] redmine_list_time_entries                    │   │
│  │  #[tool] redmine_create_time_entry                    │   │
│  └──────────────────────────┬────────────────────────────┘   │
│                             │                                │
│  ┌──────────────────────────▼────────────────────────────┐   │
│  │  RedmineClient (client.rs)                            │   │
│  │  Wrapper над redmine_api::RedmineAsync                │   │
│  └──────────────────────────┬────────────────────────────┘   │
│                             │                                │
│  ┌──────────────────────────▼────────────────────────────┐   │
│  │  Config (config.rs)  ◄──  CLI args / ENV / .env       │   │
│  └───────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐   │
│  │  CoreError (error.rs) — единый тип ошибок             │   │
│  │  RedmineApi | Reqwest | Json | Url | Config | Builder │   │
│  └───────────────────────────────────────────────────────┘   │
└─────────────────────────┬────────────────────────────────────┘
                          │
┌─────────────────────────▼────────────────────────────────────┐
│                   Redmine REST API                           │
│                   (issues, projects, users, time_entries)    │
└──────────────────────────────────────────────────────────────┘
```

## Workspace Cargo

Проект состоит из трёх крейтов в едином workspace:

| Крейт | Тип | Назначение |
|-------|-----|-----------|
| `redmine-types` | library | Типы данных (параметры запросов) |
| `redmine-core` | library | Бизнес-логика, клиент Redmine API |
| `redmine-mcp` | binary | Точка входа, MCP-сервер |

### Зависимости между крейтами

```
redmine-types  (нет внутренних зависимостей)
       ↑
redmine-core  (зависит от redmine-types)
       ↑
redmine-mcp   (зависит от redmine-types, redmine-core)
```

## Поток данных

1. **main.rs** → `Config::parse()` загружает настройки (CLI > ENV > .env)
2. **main.rs** → `RedmineClient::new(config)` создаёт HTTP-клиент
3. **main.rs** → `RedmineServer::new(client)` создаёт MCP-сервер
4. **main.rs** → `server.serve(stdio())` запускает сервер на stdin/stdout
5. MCP-клиент отправляет JSON-RPC запросы → `rmcp` маршрутизирует к `#[tool]` методам
6. Методы `RedmineServer` вызывают соответствующие методы `RedmineClient`
7. `RedmineClient` использует `redmine-api` для вызова REST API Redmine
8. Результаты возвращаются как JSON-строки → MCP-клиенту
