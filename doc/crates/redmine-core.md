# redmine-core

Библиотека бизнес-логики: клиент Redmine API, конфигурация, обработка ошибок.

**Зависимости**: `redmine-types`, `redmine-api`, `reqwest`, `clap`, `serde`, `thiserror`, `tokio`, `url`, `time`, `dotenvy`

## RedmineClient

Основной клиент для взаимодействия с Redmine REST API. Реализует все CRUD-операции.

```rust
pub struct RedmineClient {
    inner: Arc<RedmineAsync>,  // клонирование дёшево
}
```

### Создание

- `RedmineClient::new(config: Config)` — из явной конфигурации
- `RedmineClient::from_env()` — из переменных окружения

### Методы

| Метод | Описание | Возврат |
|-------|----------|---------|
| `list_issues(...)` | Список задач с фильтрами | `Result<String, CoreError>` — JSON |
| `get_issue(id)` | Задача по ID | `Result<String, CoreError>` — JSON |
| `create_issue(...)` | Создание задачи | `Result<String, CoreError>` — JSON |
| `update_issue(...)` | Обновление задачи | `Result<(), CoreError>` |
| `delete_issue(id)` | Удаление задачи | `Result<(), CoreError>` |
| `list_projects(limit, offset)` | Список проектов | `Result<String, CoreError>` — JSON |
| `get_project(id)` | Проект по ID | `Result<String, CoreError>` — JSON |
| `list_users(limit, offset)` | Список пользователей | `Result<String, CoreError>` — JSON |
| `get_user(id)` | Пользователь по ID | `Result<String, CoreError>` — JSON |
| `list_time_entries(...)` | Список записей времени | `Result<String, CoreError>` — JSON |
| `create_time_entry(...)` | Создание записи времени | `Result<String, CoreError>` — JSON |

### Особенности

- User-Agent: `redmine-mcp/0.1.0`
- Прокси отключён (`no_proxy()`)
- Возвращаемые JSON-строки содержат pretty-print (через `serde_json::to_string_pretty`)

### Хелперы

- `parse_issue_status_filter(s)` — парсит строку статуса в `IssueStatusFilter`. `"open"`, `"closed"`, `"all"`, числовой ID; неизвестные значения → `Open`.
- `parse_date(s)` — парсит дату `YYYY-MM-DD` в `time::Date`.

## Config

Конфигурация из CLI-аргументов и переменных окружения.

```rust
pub struct Config {
    pub redmine_url: String,      // --redmine-url / REDMINE_URL
    pub redmine_api_key: String,  // --redmine-api-key / REDMINE_API_KEY
}
```

**Особенности**:
- `Debug`-формат скрывает API-ключ (`***REDACTED***`)
- `Config::parse()` загружает `.env` файл автоматически
- `Config::parse_from(args)` для тестов

## CoreError

Единый тип ошибок.

```rust
pub enum CoreError {
    RedmineApi(redmine_api::Error),    // ошибки Redmine API
    Reqwest(reqwest::Error),           // HTTP-ошибки
    Json(serde_json::Error),           // ошибки JSON
    Url(url::ParseError),              // ошибки парсинга URL
    Config(String),                    // ошибки конфигурации
    Builder(String),                   // ошибки билдеров redmine-api
    TimeParse(String, ParseError),     // ошибки парсинга даты
}
```

Все builder-ошибки из `redmine-api` автоматически конвертируются в `CoreError::Builder` через `From` impls (12 реализаций).
