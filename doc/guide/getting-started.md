# Быстрый старт

## Требования

- Rust >= 1.85 (edition 2024)
- Доступ к Redmine-серверу с API-ключом

## Установка

```bash
git clone <repo-url>
cd redmine-mcp
```

## Конфигурация

Скопируйте и заполните `.env` файл:

```bash
cp .env.example .env
```

Отредактируйте `.env`:

```env
REDMINE_URL=https://your-redmine-instance.com
REDMINE_API_KEY=your_api_key_here
```

## Сборка

```bash
cargo build --release
```

## Запуск

Через переменные окружения:

```bash
REDMINE_URL="https://redmine.example.com" \
REDMINE_API_KEY="секретный_ключ" \
cargo run --release
```

Через `.env` файл:

```bash
# настройки в .env будут загружены автоматически
cargo run --release
```

Через CLI-аргументы:

```bash
cargo run --release -- \
  --redmine-url "https://redmine.example.com" \
  --redmine-api-key "секретный_ключ"
```

## Проверка

После запуска сервер слушает на **stdin/stdout** (stdio transport). MCP-клиент (например, AI-ассистент) может подключаться и вызывать инструменты:

- `redmine_list_issues`
- `redmine_create_issue`
- `redmine_list_projects`
- и другие

## Пример использования (MCP-клиент)

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "redmine_list_issues",
    "arguments": {
      "project_id": 1,
      "status_id": "open",
      "limit": 10
    }
  }
}
```
