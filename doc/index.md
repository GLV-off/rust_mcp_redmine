# redmine-mcp

**MCP-сервер для Redmine** — мост между AI-ассистентами (совместимыми с протоколом MCP) и Redmine REST API.

## Возможности

- **Issues**: просмотр, создание, обновление, удаление задач
- **Projects**: просмотр списка и деталей проектов
- **Users**: просмотр списка и деталей пользователей
- **Time entries**: просмотр и создание записей времени

## Быстрые ссылки

| Раздел | Описание |
|--------|----------|
| [Быстрый старт](guide/getting-started.md) | Установка, настройка, запуск |
| [Конфигурация](guide/configuration.md) | Параметры CLI, переменные окружения, .env |
| [Тестирование](guide/testing.md) | Стратегия, запуск, написание тестов |
| [Архитектура](architecture.md) | Структура workspace, модули, потоки данных |
| [redmine-types](crates/redmine-types.md) | Типы данных для параметров API |
| [redmine-core](crates/redmine-core.md) | Core-клиент, конфиг, обработка ошибок |
| [redmine-mcp](crates/redmine-mcp.md) | MCP-сервер и инструменты |
| [Разработка](development.md) | Сборка, соглашения, CI |

## Структура проекта

```
redmine-mcp/
├── crates/
│   ├── redmine-types/    # Типы данных (параметры запросов)
│   ├── redmine-core/     # Бизнес-логика (клиент, конфиг, ошибки)
│   └── redmine-mcp/      # MCP-сервер (точка входа, инструменты)
├── doc/                  # Документация (вы здесь)
├── .env.example          # Шаблон конфигурации
└── Cargo.toml            # Workspace root
```

## Технологии

- **Язык**: Rust (edition 2024)
- **MCP**: rmcp v2 (Model Context Protocol)
- **HTTP**: reqwest + redmine-api v0.11
- **Конфигурация**: clap (CLI) + dotenvy (.env)
- **Тестирование**: wiremock (интеграционные тесты)
