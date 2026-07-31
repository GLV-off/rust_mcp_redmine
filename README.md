# redmine-mcp

**MCP-сервер для Redmine** — мост между AI-ассистентами и Redmine REST API.

Предоставляет инструменты для работы с задачами (issues), проектами, пользователями и временными записями через протокол [MCP](https://modelcontextprotocol.io).

## Быстрый старт

```bash
cp .env.example .env
# отредактируйте REDMINE_URL и REDMINE_API_KEY в .env
cargo run --release
```

```bat
copy .env.example .env
cargo run --release
```

## Документация

Полная документация — в папке [doc/](doc/index.md):

| Раздел | Содержание |
|--------|-----------|
| [Быстрый старт](doc/guide/getting-started.md) | Установка, настройка, запуск |
| [Архитектура](doc/architecture.md) | Структура проекта и потоки данных |
| [MCP-инструменты](doc/crates/redmine-mcp.md) | Все 11 инструментов с аргументами |
| [redmine-core](doc/crates/redmine-core.md) | API клиента и конфигурация |
| [redmine-types](doc/crates/redmine-types.md) | Типы данных параметров |
| [Тестирование](doc/guide/testing.md) | Запуск и написание тестов |
| [Разработка](doc/development.md) | Сборка и соглашения |

## Лицензия

MIT
