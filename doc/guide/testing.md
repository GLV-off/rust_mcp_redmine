# Тестирование

## Стратегия

Проект использует два уровня тестов:

### Unit-тесты

Расположены в каждом крейте в `#[cfg(test)] mod tests`:

| Крейт | Кол-во тестов | Что тестируют |
|-------|--------------|---------------|
| `redmine-types` | 12 | Сериализация/десериализация всех структур данных |
| `redmine-core` | 16 | Парсинг статусов, дат, билдеры, обработка ошибок |
| `redmine-core` (config) | 5 | Debug-формат, clone, CLI-парсинг |
| `redmine-core` (error) | 7 | Display, Debug, From-конверсии, Send+Sync |
| `redmine-mcp` | 18 | Десериализация аргументов всех инструментов |

### Интеграционные тесты

Расположены в `crates/redmine-core/tests/client_integration.rs` (23 теста).

Используют **wiremock** для эмуляции Redmine API без реального сервера.

Покрытие:
- **Issues**: list (с фильтрами, пустой, пагинация), get, create (minimal, full), update, delete
- **Projects**: list (с данными, пустой), get
- **Users**: list, get
- **Time entries**: list (с фильтрами), create (full, minimal)
- **Auth**: проверка заголовка `X-Redmine-API-Key`
- **Errors**: 404, 401, 403, 500, 422 validation

## Запуск тестов

```bash
# Все тесты
cargo test

# Только unit-тесты (без интеграционных)
cargo test --lib

# Только интеграционные тесты
cargo test --test client_integration

# Тесты конкретного крейта
cargo test -p redmine-types
cargo test -p redmine-core
cargo test -p redmine-mcp

# С фильтром по имени
cargo test test_list_issues
```

## Написание новых тестов

### Unit-тест для нового типа данных

```rust
#[test]
fn test_new_params() {
    let params = NewParams { field: Some(42) };
    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["field"], 42);

    let deserialized: NewParams = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.field, Some(42));
}
```

### Интеграционный тест для нового API-метода

```rust
#[tokio::test]
async fn test_new_operation() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/new_endpoint.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({...})))
        .mount(&mock)
        .await;

    let result = client.some_operation(args).await;
    assert!(result.is_ok());
}
```
