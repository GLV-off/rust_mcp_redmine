# redmine-types

Пакет типов данных для параметров Redmine API.

**Зависимости**: `serde`, `schemars`

## Issues

### `ListIssuesParams`

Параметры фильтрации списка задач.

| Поле | Тип | Описание |
|------|-----|----------|
| `project_id` | `Option<u64>` | Фильтр по ID проекта |
| `status_id` | `Option<String>` | Фильтр по статусу: `"open"`, `"closed"`, `"all"` или числовой ID |
| `tracker_id` | `Option<u64>` | Фильтр по ID трекера |
| `assignee_id` | `Option<u64>` | Фильтр по ID исполнителя |
| `limit` | `Option<u64>` | Максимум результатов на странице (по умолч. 25, макс. 100) |
| `offset` | `Option<u64>` | Смещение для пагинации |

### `GetIssueParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `id` | `u64` | ID задачи |

### `CreateIssueParams`

Обязательные поля: `project_id`, `subject`.

| Поле | Тип | Описание |
|------|-----|----------|
| `project_id` | `u64` | ID проекта |
| `tracker_id` | `Option<u64>` | ID трекера |
| `status_id` | `Option<u64>` | ID статуса |
| `priority_id` | `Option<u64>` | ID приоритета |
| `subject` | `String` | Тема задачи |
| `description` | `Option<String>` | Описание задачи |
| `assigned_to_id` | `Option<u64>` | ID исполнителя |
| `parent_issue_id` | `Option<u64>` | ID родительской задачи (подзадачи) |
| `estimated_hours` | `Option<f64>` | Оценка времени в часах |

### `UpdateIssueParams`

Все поля кроме `id` опциональны — обновляются только указанные поля.

| Поле | Тип | Описание |
|------|-----|----------|
| `id` | `u64` | ID задачи (обязательно) |
| `project_id` | `Option<u64>` | Новый проект |
| `tracker_id` | `Option<u64>` | Новый трекер |
| `status_id` | `Option<u64>` | Новый статус |
| `priority_id` | `Option<u64>` | Новый приоритет |
| `subject` | `Option<String>` | Новая тема |
| `description` | `Option<String>` | Новое описание |
| `assigned_to_id` | `Option<u64>` | Новый исполнитель |
| `parent_issue_id` | `Option<u64>` | Новая родительская задача |
| `estimated_hours` | `Option<f64>` | Новая оценка времени |
| `notes` | `Option<String>` | Комментарий к обновлению |

### `DeleteIssueParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `id` | `u64` | ID задачи для удаления |

## Projects

### `ListProjectsParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `limit` | `Option<u64>` | Максимум результатов на странице (по умолч. 25, макс. 100) |
| `offset` | `Option<u64>` | Смещение для пагинации |

### `GetProjectParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `id` | `u64` | ID проекта |

## Users

### `ListUsersParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `limit` | `Option<u64>` | Максимум результатов на странице (по умолч. 25, макс. 100) |
| `offset` | `Option<u64>` | Смещение для пагинации |

### `GetUserParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `id` | `u64` | ID пользователя |

## Time Entries

### `ListTimeEntriesParams`

| Поле | Тип | Описание |
|------|-----|----------|
| `project_id` | `Option<String>` | Фильтр по ID или идентификатору проекта |
| `spent_on` | `Option<String>` | Фильтр по дате (`YYYY-MM-DD`) |
| `limit` | `Option<u64>` | Максимум результатов на странице |
| `offset` | `Option<u64>` | Смещение для пагинации |

### `CreateTimeEntryParams`

`hours` обязателен. Требуется `issue_id` или `project_id`.

| Поле | Тип | Описание |
|------|-----|----------|
| `issue_id` | `Option<u64>` | ID задачи |
| `project_id` | `Option<u64>` | ID проекта (если нет issue_id) |
| `hours` | `f64` | Количество часов (обязательно) |
| `activity_id` | `Option<u64>` | ID активности |
| `comments` | `Option<String>` | Комментарий |
| `spent_on` | `Option<String>` | Дата (`YYYY-MM-DD`, по умолч. сегодня) |
