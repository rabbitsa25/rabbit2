# API de Produtos - Tauri/Rust

Ponte entre o frontend (React/TypeScript) e o backend (Rust/Tauri) para gerenciamento de produtos.

Esta implementação replica a funcionalidade do `ProdutosController` do NestJS, mas usando comandos Tauri ao invés de endpoints HTTP.

## 📋 Estrutura da Entidade

```typescript
interface Product {
    id?: number;
    code: string;        // Código do produto
    name: string;        // Nome do produto
    active: number;      // 1 = ativo, 0 = inativo
    balance: number;     // Saldo/estoque (decimal com 4 casas)
    created_at: string;  // Data de criação (ISO 8601)
    updated_at: string;  // Data de atualização (ISO 8601)
}
```

## 🔌 Comandos Disponíveis

### POST /products - Criar Produto
```typescript
await ProductsApi.create(code: string, name: string): Promise<Product>
```

**Exemplo:**
```typescript
const product = await ProductsApi.create('PROD001', 'Produto Teste');
console.log(product);
// { id: 1, code: 'PROD001', name: 'Produto Teste', active: 1, balance: 0.0000, ... }
```

---

### GET /products/:id - Buscar por ID
```typescript
await ProductsApi.getById(id: number): Promise<Product | null>
```

**Exemplo:**
```typescript
const product = await ProductsApi.getById(1);
if (product) {
    console.log(`Produto: ${product.name}`);
}
```

---

### GET /products/code/:code - Buscar por Código
```typescript
await ProductsApi.getByCode(code: string): Promise<Product | null>
```

**Exemplo:**
```typescript
const product = await ProductsApi.getByCode('PROD001');
if (product) {
    console.log(`Encontrado: ${product.name}`);
}
```

---

### GET /products - Listar Todos
```typescript
await ProductsApi.getAll(): Promise<Product[]>
```

**Exemplo:**
```typescript
const products = await ProductsApi.getAll();
console.log(`Total de produtos: ${products.length}`);
```

---

### GET /products (ativos apenas)
```typescript
await ProductsApi.getAllActive(): Promise<Product[]>
```

**Exemplo:**
```typescript
const activeProducts = await ProductsApi.getAllActive();
console.log(`Produtos ativos: ${activeProducts.length}`);
```

---

### PUT /products/:id - Atualizar Produto
```typescript
await ProductsApi.update(
    id: number, 
    data: {
        code?: string;
        name?: string;
        active?: number;
        balance?: number;
    }
): Promise<Product>
```

**Exemplo:**
```typescript
const updated = await ProductsApi.update(1, {
    name: 'Novo Nome',
    balance: 100.5
});
console.log(updated);
```

---

### DELETE /products/:id - Deletar Produto
```typescript
await ProductsApi.delete(id: number): Promise<void>
```

**Nota:** Realiza um *soft delete* (marca como inativo, não remove do banco).

**Exemplo:**
```typescript
await ProductsApi.delete(1);
console.log('Produto deletado (marcado como inativo)');
```

---

### PATCH /products/:id/increment - Incrementar Saldo
```typescript
await ProductsApi.incrementBalance(id: number, amount: number): Promise<Product>
```

**Exemplo:**
```typescript
const product = await ProductsApi.incrementBalance(1, 50.75);
console.log(`Novo saldo: ${product.balance}`);
```

---

### PATCH /products/:id/decrement - Decrementar Saldo
```typescript
await ProductsApi.decrementBalance(id: number, amount: number): Promise<Product>
```

**Exemplo:**
```typescript
const product = await ProductsApi.decrementBalance(1, 25.5);
console.log(`Saldo após desconto: ${product.balance}`);
```

---

## 🎯 Exemplo de Uso Completo

```typescript
import { ProductsApi } from './api/products';

async function exemploCompleto() {
    try {
        // 1. Criar produto
        const newProduct = await ProductsApi.create('SKU123', 'Produto Exemplo');
        console.log('Criado:', newProduct);

        // 2. Incrementar saldo
        const withStock = await ProductsApi.incrementBalance(newProduct.id!, 100);
        console.log('Saldo adicionado:', withStock.balance); // 100.0000

        // 3. Buscar por código
        const found = await ProductsApi.getByCode('SKU123');
        console.log('Encontrado:', found);

        // 4. Atualizar nome
        const updated = await ProductsApi.update(newProduct.id!, {
            name: 'Produto Atualizado'
        });
        console.log('Atualizado:', updated.name);

        // 5. Decrementar saldo
        const afterSale = await ProductsApi.decrementBalance(newProduct.id!, 30);
        console.log('Saldo após venda:', afterSale.balance); // 70.0000

        // 6. Listar todos
        const all = await ProductsApi.getAll();
        console.log('Total:', all.length);

        // 7. Deletar (soft delete)
        await ProductsApi.delete(newProduct.id!);
        console.log('Deletado (inativo)');

    } catch (error) {
        console.error('Erro:', error);
    }
}
```

---

## 🔄 Mapeamento com NestJS Controller

| NestJS Endpoint | Tauri Command | Método API |
|----------------|---------------|------------|
| `POST /products` | `create_product` | `ProductsApi.create()` |
| `GET /products/:id` | `get_product` | `ProductsApi.getById()` |
| `GET /products/code/:code` | `get_product_by_code` | `ProductsApi.getByCode()` |
| `GET /products` | `get_all_products` | `ProductsApi.getAll()` |
| `PUT /products/:id` | `update_product` | `ProductsApi.update()` |
| `DELETE /products/:id` | `delete_product` | `ProductsApi.delete()` |
| `PATCH /products/:id/increment` | `increment_product_balance` | `ProductsApi.incrementBalance()` |
| `PATCH /products/:id/decrement` | `decrement_product_balance` | `ProductsApi.decrementBalance()` |

---

## 🛠️ Implementação Backend (Rust)

### Serviço: `ProductService`

Localizado em: `src-tauri/src/services/product_service.rs`

```rust
impl ProductService {
    pub fn create(code: String, name: String) -> Result<ProductEntity, String>
    pub fn find_by_id(id: i64) -> Result<Option<ProductEntity>, String>
    pub fn find_by_code(code: &str) -> Result<Option<ProductEntity>, String>
    pub fn find_all() -> Result<Vec<ProductEntity>, String>
    pub fn find_all_active() -> Result<Vec<ProductEntity>, String>
    pub fn update(...) -> Result<ProductEntity, String>
    pub fn delete(id: i64) -> Result<(), String>
    pub fn increment_balance(id: i64, amount: f64) -> Result<ProductEntity, String>
    pub fn decrement_balance(id: i64, amount: f64) -> Result<ProductEntity, String>
}
```

### Comandos Tauri

Localizado em: `src-tauri/src/lib.rs`

```rust
#[tauri::command]
fn create_product(code: String, name: String) -> Result<ProductEntity, String>

#[tauri::command]
fn get_product(id: i64) -> Result<Option<ProductEntity>, String>

// ... outros comandos
```

---

## 📊 Banco de Dados

Tabela: `produtos`

```sql
CREATE TABLE produtos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    active INTEGER DEFAULT 1 NOT NULL,
    balance REAL DEFAULT 0.0000 NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

---

## ⚠️ Notas Importantes

1. **Soft Delete**: O método `delete()` apenas marca o produto como inativo (`active = 0`), não remove do banco
2. **Saldo Decimal**: O campo `balance` aceita até 4 casas decimais (tipo `REAL` no SQLite)
3. **Active Flag**: Usar `1` para ativo, `0` para inativo (integer, não boolean)
4. **Timestamps**: Armazenados como strings ISO 8601 (`YYYY-MM-DDTHH:MM:SS.sssZ`)

---

## 🚀 Como Usar no Projeto

1. Importe a API no seu componente:
```typescript
import { ProductsApi } from '@/api/products';
```

2. Use as funções assíncronas:
```typescript
const products = await ProductsApi.getAll();
```

3. Trate erros adequadamente:
```typescript
try {
    const product = await ProductsApi.create('CODE', 'Name');
} catch (error) {
    console.error('Falha ao criar produto:', error);
}
```
