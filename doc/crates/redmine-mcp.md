# redmine-mcp

MCP-сервер и точка входа приложения.

**Зависимости**: `redmine-core`, `redmine-types`, `rmcp`, `tokio`, `serde`, `schemars`, `anyhow`

## Точка входа

`main.rs` — бинарный крейт:

1. Парсит конфигурацию (`Config::parse()`)
2. Создаёт `RedmineClient`
3. Создаёт `RedmineServer`
4. Запускает MCP-сервер на stdio транспорте (`rmcp::transport::stdio`)

## RedmineServer

Структура, реализующая MCP-инструменты через макрос `#[tool_router]`.

```rust
pub struct RedmineServer {
    client: Arc<RedmineClient>,
}
```

## MCP-инструменты

Всего **11 инструментов**.

### Issues

| Инструмент | Описание | Ключевые аргументы |
|-----------|----------|-------------------|
| `redmine_list_issues` | Список задач с фильтрами | `project_id`, `status_id`, `tracker_id`, `assignee_id`, `limit`, `offset` |
| `redmine_get_issue` | Задача по ID | `id` |
| `redmine_create_issue` | Создание задачи | `project_id`, `subject` (+ 7 опциональных) |
| `redmine_update_issue` | Обновление задачи | `id` (+ 10 опциональных) |
| `redmine_delete_issue` | Удаление задачи | `id` → `{"status":"deleted","id":...}` |

### Projects

| Инструмент | Описание | Ключевые аргументы |
|-----------|----------|-------------------|
| `redmine_list_projects` | Список проектов | `limit`, `offset` |
| `redmine_get_project` | Проект по ID | `id` |

### Users

| Инструмент | Описание | Ключевые аргументы |
|-----------|----------|-------------------|
| `redmine_list_users` | Список пользователей | `limit`, `offset` |
| `redmine_get_user` | Пользователь по ID | `id` |

### Time Entries

| Инструмент | Описание | Ключевые аргументы |
|-----------|----------|-------------------|
| `redmine_list_time_entries` | Список записей времени | `project_id`, `spent_on`, `limit`, `offset` |
| `redmine_create_time_entry` | Создание записи времени | `hours` (+ 5 опциональных), `issue_id` или `project_id` |

## Обработка ошибок

Все инструменты возвращают ошибки MCP через `McpError::internal_error` с текстовым описанием.
