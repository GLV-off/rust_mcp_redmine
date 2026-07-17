# rst_mcp_redmine

MCP-сервер для Redmine. Позволяет AI-агентам (в т.ч. OpenCode) взаимодействовать с Redmine через протокол MCP (Model Context Protocol).

---

## О проекте

`rst_mcp_redmine` — это Rust-приложение, реализующее MCP-сервер для Redmine. Он предоставляет агентам набор инструментов для работы с задачами (issues), проектами, пользователями и учётом времени (time entries) в Redmine.

**Назначение:** Интеграция Redmine с AI-помощниками через MCP. Агент может создавать задачи, обновлять статусы, искать проекты, логировать время и т.д. прямо из диалога.

**Структура репозитория:**

```
rst_mcp_redmine/
├── .cargo/                    # Конфигурация Cargo
├── .opencode/                 # Агенты OpenCode (architect, dev, qa_tester, tech_writer)
├── crates/
│   ├── redmine-types/         # Модели данных и параметры API
│   ├── redmine-core/          # HTTP-клиент, конфиг, обработка ошибок
│   └── redmine-mcp/           # MCP-сервер, инструменты (tools)
├── doc/                       # Документация
├── Cargo.toml                 # Workspace root
├── opencode.json               # Конфигурация команд OpenCode
└── .env.example               # Пример переменных окружения
```

---

## Архитектура

Проект состоит из трёх крейтов (Rust-пакетов) в одном workspace:

### 1. `redmine-types`
Модели данных и структуры параметров запросов. Используется обоими другими крейтами.

- `issues.rs` — `ListIssuesParams`, `CreateIssueParams`, `UpdateIssueParams`, `DeleteIssueParams`
- `projects.rs` — `ListProjectsParams`, `GetProjectParams`
- `users.rs` — `ListUsersParams`, `GetUserParams`
- `time_entries.rs` — `ListTimeEntriesParams`, `CreateTimeEntryParams`

### 2. `redmine-core`
Реализация бизнес-логики и HTTP-взаимодействия с Redmine API.

- `client.rs` — `RedmineClient`: обёртка над `redmine-api`, методы для всех CRUD-операций
- `config.rs` — `Config`: парсинг конфигурации из CLI-аргументов и переменных окружения (через `clap`)
- `error.rs` — `CoreError`: иерархия ошибок с `thiserror`

### 3. `redmine-mcp`
MCP-сервер на базе библиотеки [`rmcp`](https://crates.io/crates/rmcp). Запускается через STDIO транспорт.

- `main.rs` — точка входа, инициализация и запуск сервера
- `tools.rs` — определение MCP-инструментов и их обработчиков

### Поток данных

```
AI Agent (OpenCode)
    │
    ▼  MCP (STDIO)
redmine-mcp (сервер)
    │
    ▼
redmine-core (HTTP клиент)
    │
    ▼
Redmine REST API (https://your-redmine.com)
```

### MCP-инструменты

| Инструмент | Описание |
|---|---|
| `redmine_list_issues` | Список задач с фильтрами (проект, статус, трекер, ответственный) |
| `redmine_get_issue` | Получить задачу по ID |
| `redmine_create_issue` | Создать новую задачу |
| `redmine_update_issue` | Обновить задачу |
| `redmine_delete_issue` | Удалить задачу |
| `redmine_list_projects` | Список проектов |
| `redmine_get_project` | Получить проект по ID |
| `redmine_list_users` | Список пользователей |
| `redmine_get_user` | Получить пользователя по ID |
| `redmine_list_time_entries` | Записи времени с фильтрами |
| `redmine_create_time_entry` | Создать запись времени |

---

## Быстрый старт

### Требования

- Rust 1.85+ (MSRV)
- Доступ к Redmine-инстансу с API-ключом

### Установка и запуск

```bash
# 1. Клонировать репозиторий
git clone <url>
cd rst_mcp_redmine

# 2. Настроить окружение
cp .env.example .env
# Отредактировать .env:
#   REDMINE_URL=https://redmine.example.com
#   REDMINE_API_KEY=your_api_key_here

# 3. Собрать
cargo build --release

# 4. Запустить MCP-сервер (STDIO режим)
cargo run -p redmine-mcp
```

### Переменные окружения

| Переменная | Обязательная | Описание |
|---|---|---|
| `REDMINE_URL` | Да | URL Redmine (e.g. `https://redmine.example.com`) |
| `REDMINE_API_KEY` | Да | API-ключ Redmine |

Можно также передать через CLI-аргументы:
```bash
cargo run -p redmine-mcp -- --redmine-url https://... --redmine-api-key ...
```

### Команды (через opencode.json)

```bash
cargo build           # Сборка
cargo test            # Тестирование
cargo clippy --all-targets -- -D warnings  # Линтер
cargo run -p redmine-mcp  # Запуск сервера
```

---

## Интеграция с OpenCode

Для того чтобы OpenCode-агент мог использовать Redmine через этот MCP-сервер, необходимо:

### Шаг 1: Собрать сервер

```bash
cargo build --release
```

Бинарный файл будет расположен в `target/release/redmine-mcp.exe`.

### Шаг 2: Добавить MCP-сервер в OpenCode

В файл `~/.config/opencode/opencode.jsonc` (глобально) или в `opencode.json` вашего проекта добавить:

```jsonc
{
  "$schema": "https://opencode.ai/config.json",
  "mcpServers": {
    "redmine": {
      "command": "C:\\Dev\\GLv\\rst_mcp_redmine\\target\\release\\redmine-mcp.exe",
      "env": {
        "REDMINE_URL": "https://redmine.example.com",
        "REDMINE_API_KEY": "your_api_key_here"
      }
    }
  }
}
```

Или, если сервер уже добавлен в `PATH`:
```jsonc
{
  "mcpServers": {
    "redmine": {
      "command": "redmine-mcp",
      "env": {
        "REDMINE_URL": "https://redmine.example.com",
        "REDMINE_API_KEY": "your_api_key_here"
      }
    }
  }
}
```

### Шаг 3: Проверка

После добавления, при следующем запуске OpenCode агенты автоматически получат доступ к инструментам `redmine_*`. Можно проверить в диалоге:
- "Покажи список проектов в Redmine"
- "Создай задачу в проекте 1 с темой 'Тест'"
- "Найди задачи со статусом open"

---

## Навык (Skill) для агента OpenCode

Для оптимальной работы агента с Redmine рекомендуется создать навык. Навык описывает агенту,
как и когда использовать инструменты Redmine.

### Создание навыка

Создайте файл `redmine-mcp.md` в папке навыков OpenCode (напр. `~/.config/opencode/skills/` или `.opencode/skills/` проекта):

```markdown
# Redmine MCP Skill

Используй инструменты Redmine когда пользователь просит:
- Работать с задачами (issues)
- Смотреть или создавать проекты
- Управлять пользователями
- Логировать время

## Доступные операции

### Issues
- `redmine_list_issues(project_id?, status_id?, tracker_id?, assignee_id?, limit?, offset?)` — список задач. status_id: "open"|"closed"|"all"|число
- `redmine_get_issue(id)` — задача по ID
- `redmine_create_issue(project_id, subject, description?, tracker_id?, status_id?, priority_id?, assigned_to_id?, parent_issue_id?, estimated_hours?)` — создать
- `redmine_update_issue(id, subject?, description?, status_id?, priority_id?, assigned_to_id?, estimated_hours?, notes?, tracker_id?, parent_issue_id?, project_id?)` — обновить
- `redmine_delete_issue(id)` — удалить

### Projects
- `redmine_list_projects(limit?, offset?)` — список
- `redmine_get_project(id)` — по ID

### Users
- `redmine_list_users(limit?, offset?)` — список
- `redmine_get_user(id)` — по ID

### Time Entries
- `redmine_list_time_entries(project_id?, spent_on?, limit?, offset?)` — записи времени. spent_on: "YYYY-MM-DD"
- `redmine_create_time_entry(issue_id?, project_id?, hours, activity_id?, comments?, spent_on?)` — создать запись

## Примечания
- Всегда проверяй project_id перед созданием задачи
- При обновлении задачи можно передать `notes` для добавления комментария
- Для time entries обязателен `issue_id` или `project_id`
- По умолчанию limit=25, offset=0
```

### Подключение навыка

В `opencode.json` добавьте ссылку на навык:

```jsonc
{
  "mcpServers": {
    "redmine": {
      "command": "target/release/redmine-mcp.exe",
      "env": {
        "REDMINE_URL": "...",
        "REDMINE_API_KEY": "..."
      }
    }
  },
  "skills": [
    {
      "name": "redmine",
      "description": "Интеграция с Redmine через MCP",
      "path": ".opencode/skills/redmine-mcp.md"
    }
  ]
}
```

После этого агенты OpenCode смогут эффективно использовать Redmine, зная все доступные инструменты и их параметры.

---

## Разработка

### Тестирование

```bash
# Все тесты
cargo test

# Только unit-тесты
cargo test --lib

# Интеграционные тесты
cargo test --test client_integration
```

### Добавление нового инструмента

1. Определить параметры в `crates/redmine-types/src/<domain>.rs`
2. Реализовать метод в `crates/redmine-core/src/client.rs`
3. Добавить MCP-инструмент в `crates/redmine-mcp/src/tools.rs`
4. Написать тесты (unit + интеграционные)
5. Обновить документацию

### Структура ошибок

Все ошибки проходят через `CoreError` (enum с `thiserror`). В MCP-слое они преобразуются в `McpError::internal_error` и возвращаются агенту с описанием.

---

## Лицензия

MIT
