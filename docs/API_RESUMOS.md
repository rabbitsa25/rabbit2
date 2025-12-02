# API de Resumos

Documentação das rotas de resumos de pagamentos disponíveis no servidor HTTP (porta 8088).

## Base URL
```
http://localhost:8088/resumes
```

---

## Endpoints

### 1. **GET /** 
Busca todos os resumos de pagamentos do dia atual.

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "code": "Dinheiro",
    "amount_s": 1500.50,
    "amount_n": 250.00,
    "updated_at": 1701436800000,
    "created_at": 1701388800000
  },
  {
    "id": "660e8400-e29b-41d4-a716-446655440001",
    "code": "CartaoDeCredito",
    "amount_s": 3200.75,
    "amount_n": 0.00,
    "updated_at": 1701437100000,
    "created_at": 1701388800000
  }
]
```

---

## Estrutura da Entidade

### ResumeEntity
Representa um resumo de pagamentos agrupados por tipo.

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `id` | string (UUID) | ID único do resumo gerado com UUID v4 |
| `code` | PaymentTypes | Tipo de pagamento (enum) |
| `amount_s` | f64 | Valor total sincronizado com o servidor |
| `amount_n` | f64 | Valor total não sincronizado |
| `updated_at` | i64 | Timestamp de atualização em milissegundos |
| `created_at` | i64 | Timestamp de criação em milissegundos |

---

## PaymentTypes Enum

Enum que representa os tipos de pagamento conforme padrão NF-e.

### Valores Possíveis

| Valor JSON | Código | Descrição |
|------------|--------|-----------|
| `"Dinheiro"` | 01 | Dinheiro |
| `"Cheque"` | 02 | Cheque |
| `"CartaoDeCredito"` | 03 | Cartão de Crédito |
| `"CartaoDeDebito"` | 04 | Cartão de Débito |
| `"CreditoLoja"` | 05 | Crédito Loja |
| `"ValeAlimentacao"` | 10 | Vale Alimentação |
| `"ValeRefeicao"` | 11 | Vale Refeição |
| `"ValePresente"` | 12 | Vale Presente |
| `"ValeCombustivel"` | 13 | Vale Combustível |
| `"DuplicataMercantil"` | 14 | Duplicata Mercantil |
| `"BoletoBancario"` | 15 | Boleto Bancário |
| `"SemPagamento"` | 90 | Sem pagamento |
| `"Outros"` | 99 | Outros |

---

## Funcionalidades do ResumeService

O `ResumeService` oferece os seguintes métodos (disponíveis para uso interno no Rust):

### `get_all_today()`
Retorna todos os resumos criados no dia atual.

### `find_by_id(id: &str)`
Busca um resumo específico por ID (UUID).

### `find_or_create_by_code(code: PaymentTypes)`
Busca ou cria um resumo para o tipo de pagamento especificado no dia atual.

### `save(resume: &ResumeEntity)`
Salva ou atualiza um resumo no banco de dados.

### `update_amounts(id: &str, amount_s: f64, amount_n: f64)`
Atualiza os valores sincronizado e não sincronizado de um resumo.

### `increment_amounts(id: &str, amount_s_inc: f64, amount_n_inc: f64)`
Incrementa os valores de um resumo (útil para acumuladores).

### `delete_old_resumes(days_old: i64)`
Remove resumos antigos (útil para manutenção do banco de dados).

---

## Casos de Uso

### 1. **Consultar Resumo Diário**
Obter o resumo de todas as formas de pagamento utilizadas no dia.

```bash
curl http://localhost:8088/resumes/
```

### 2. **Dashboard de Caixa**
Usar os dados retornados para exibir um dashboard com:
- Total em dinheiro (`amount_s` onde `code = "Dinheiro"`)
- Total em cartões (`CartaoDeCredito` + `CartaoDeDebito`)
- Total geral somando todos os `amount_s`

### 3. **Sincronização**
- `amount_s`: Valores já sincronizados com o servidor/nuvem
- `amount_n`: Valores pendentes de sincronização

---

## Exemplos de Response

### Dia com Múltiplos Pagamentos
```json
[
  {
    "id": "a1b2c3d4-e5f6-4789-a012-3456789abcde",
    "code": "Dinheiro",
    "amount_s": 2450.00,
    "amount_n": 150.50,
    "updated_at": 1701439200000,
    "created_at": 1701388800000
  },
  {
    "id": "b2c3d4e5-f6a7-4890-b123-456789abcdef",
    "code": "CartaoDeCredito",
    "amount_s": 5320.75,
    "amount_n": 0.00,
    "updated_at": 1701439260000,
    "created_at": 1701388800000
  },
  {
    "id": "c3d4e5f6-a7b8-4901-c234-56789abcdef0",
    "code": "CartaoDeDebito",
    "amount_s": 1890.25,
    "amount_n": 0.00,
    "updated_at": 1701439320000,
    "created_at": 1701388800000
  },
  {
    "id": "d4e5f6a7-b8c9-4012-d345-6789abcdef01",
    "code": "Outros",
    "amount_s": 0.00,
    "amount_n": 250.00,
    "updated_at": 1701439380000,
    "created_at": 1701388800000
  }
]
```

### Análise de Totais
```javascript
// Exemplo de processamento no frontend
const resumes = await fetch('http://localhost:8088/resumes/').then(r => r.json());

const totalSincronizado = resumes.reduce((sum, r) => sum + r.amount_s, 0);
const totalPendente = resumes.reduce((sum, r) => sum + r.amount_n, 0);
const totalGeral = totalSincronizado + totalPendente;

console.log({
  totalSincronizado,  // Ex: 9660.00
  totalPendente,      // Ex: 400.50
  totalGeral          // Ex: 10060.50
});
```

---

## Notas Técnicas

1. **Timestamp em Milissegundos**: Os campos `created_at` e `updated_at` usam timestamps Unix em milissegundos (JavaScript compatible).

2. **UUID v4**: Cada resumo tem um ID único gerado automaticamente no formato UUID v4.

3. **Agrupamento Diário**: Os resumos são criados e agrupados por dia. Um novo conjunto de resumos é iniciado à meia-noite (00:00:00).

4. **Persistência**: Os dados são armazenados no banco SQLite local.

5. **Sincronização**: O campo `amount_s` representa valores já sincronizados, enquanto `amount_n` representa valores que ainda precisam ser enviados para o servidor central.

6. **Ordenação**: Os resumos são retornados ordenados por código de pagamento.

---

## Integração com Vendas

Os resumos podem ser calculados automaticamente a partir dos pagamentos das vendas:

```rust
// Exemplo interno (Rust)
use crate::services::{VendaService, ResumeService};
use crate::entities::PaymentTypes;

// Ao finalizar uma venda
let payments = VendaService::find_payments_by_venda_id(venda_id)?;

for payment in payments {
    let code = PaymentTypes::from_str(&payment.code).unwrap();
    let resume = ResumeService::find_or_create_by_code(code)?;
    
    // Incrementa o valor não sincronizado
    ResumeService::increment_amounts(
        &resume.id, 
        0.0,  // amount_s não muda
        payment.total_pagamento  // incrementa amount_n
    )?;
}
```

---

## Status Codes

| Status | Descrição |
|--------|-----------|
| 200 | Sucesso - Resumos retornados |
| 500 | Erro interno do servidor |

---

## CORS

O endpoint possui CORS habilitado para permitir requisições de qualquer origem.
