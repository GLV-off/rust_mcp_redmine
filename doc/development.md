# Разработка

## Сборка

```bash
# Релизная сборка
cargo build --release

# Отладочная сборка
cargo build
```

## Запуск линтера и форматтера

```bash
# Форматирование
cargo fmt

# Статический анализ (clippy)
cargo clippy --all-targets -- -D warnings
```

## Тестирование

```bash
# Все тесты
cargo test

# Без интеграционных тестов
cargo test --lib

# Генерация документации
cargo doc --no-deps --open
```

## Добавление нового MCP-инструмента

1. **Типы данных** (`redmine-types`): создайте структуру с параметрами (derive `Serialize`, `Deserialize`, `JsonSchema`)
2. **Клиент** (`redmine-core/src/client.rs`): добавьте метод в `RedmineClient`
3. **Инструмент** (`redmine-mcp/src/tools.rs`):
   - Создайте структуру аргументов с `#[schemars(description = "...")]`
   - Добавьте метод в `impl RedmineServer` с `#[tool(description = "...")]`
4. **Тесты**:
   - Unit-тест на сериализацию аргументов в `tools.rs`
   - Unit-тесты на метод клиента в `client.rs`
   - Интеграционный тест с wiremock в `client_integration.rs`

## Добавление нового эндпоинта Redmine API

1. Убедитесь, что `redmine-api` поддерживает нужный эндпоинт
2. Создайте параметры в `redmine-types`
3. Реализуйте метод в `RedmineClient`
4. Добавьте MCP-инструмент в `redmine-mcp`

## Соглашения по коду

- **Rust edition 2024**, минимальная версия 1.85
- Имена файлов: `snake_case.rs`
- Публичные API документируются через `/// doc comments`
- Ошибки конвертируются через `thiserror` и `From` impls
- Асинхронные методы используют `tokio`
- Сериализация через `serde` с derive-макросами
