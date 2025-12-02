# API de Vendas

Documentação das rotas de vendas disponíveis no servidor HTTP (porta 8088).

## Base URL
```
http://localhost:8088/vendas
```

---

## Endpoints

### 1. **GET /** 
Endpoint informativo sobre vendas.

**Response:**
```json
{
  "message": "Vendas por intervalo de datas"
}
```

---

### 2. **GET /get-vendas-by-interval**
Busca vendas completas (com itens e pagamentos) por intervalo de datas.

**Query Parameters:**
- `dtInit` (string, required): Data inicial no formato `YYYY-MM-DD`
- `dtFim` (string, required): Data final no formato `YYYY-MM-DD`

**Exemplo:**
```
GET /vendas/get-vendas-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
```

**Response:**
```json
[
  {
    "venda": {
      "id": 1,
      "tip": 59,
      "mod": 65,
      "serie_origin": "0",
      "serie": "900001234",
      "nr_nf_origin": 0,
      "nr_nf": 123456,
      "cnpj": "12345678000190",
      "doc_destinatario": "12345678900",
      "dh_emi": "2024-06-15T10:30:00",
      "dh_emi_canc": null,
      "total": 150.50,
      "addition": 5.00,
      "discount": 10.00,
      "chave": "CFe12345678901234567890123456789012345678901234",
      "chave_canc": null,
      "file_path": "/path/to/nfe.xml",
      "cancel_file_path": null,
      "protocolo": "123456789012345",
      "cancelled": 0,
      "created_at": "2024-06-15T10:30:00Z",
      "updated_at": "2024-06-15T10:30:00Z"
    },
    "itens": [
      {
        "id": 1,
        "venda_id": 1,
        "produto_code": "001",
        "produto_description": "Produto Teste",
        "produto_medida": "UN",
        "quantidade": 2.0,
        "preco_unitario": 75.25,
        "desconto": 5.00,
        "desconto_rat": 0.00,
        "acrescimo": 2.50,
        "acrescimo_rat": 0.00,
        "preco_total": 145.00,
        "created_at": "2024-06-15T10:30:00Z",
        "updated_at": "2024-06-15T10:30:00Z"
      }
    ],
    "pagamentos": [
      {
        "id": 1,
        "venda_id": 1,
        "code": "01",
        "name": "Dinheiro",
        "total_pagamento": 150.50,
        "created_at": "2024-06-15T10:30:00Z",
        "updated_at": "2024-06-15T10:30:00Z"
      }
    ]
  }
]
```

---

### 3. **GET /get-items-by-interval**
Busca todos os itens de vendas realizadas no intervalo de datas.

**Query Parameters:**
- `dtInit` (string, required): Data inicial no formato `YYYY-MM-DD`
- `dtFim` (string, required): Data final no formato `YYYY-MM-DD`

**Exemplo:**
```
GET /vendas/get-items-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
```

**Response:**
```json
[
  {
    "id": 1,
    "venda_id": 1,
    "produto_code": "001",
    "produto_description": "Produto Teste",
    "produto_medida": "UN",
    "quantidade": 2.0,
    "preco_unitario": 75.25,
    "desconto": 5.00,
    "desconto_rat": 0.00,
    "acrescimo": 2.50,
    "acrescimo_rat": 0.00,
    "preco_total": 145.00,
    "created_at": "2024-06-15T10:30:00Z",
    "updated_at": "2024-06-15T10:30:00Z"
  }
]
```

---

### 4. **GET /get-payments-by-interval**
Busca todos os pagamentos de vendas realizadas no intervalo de datas.

**Query Parameters:**
- `dtInit` (string, required): Data inicial no formato `YYYY-MM-DD`
- `dtFim` (string, required): Data final no formato `YYYY-MM-DD`

**Exemplo:**
```
GET /vendas/get-payments-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
```

**Response:**
```json
[
  {
    "id": 1,
    "venda_id": 1,
    "code": "01",
    "name": "Dinheiro",
    "total_pagamento": 150.50,
    "created_at": "2024-06-15T10:30:00Z",
    "updated_at": "2024-06-15T10:30:00Z"
  },
  {
    "id": 2,
    "venda_id": 2,
    "code": "03",
    "name": "Cartão de Crédito",
    "total_pagamento": 250.00,
    "created_at": "2024-06-15T11:00:00Z",
    "updated_at": "2024-06-15T11:00:00Z"
  }
]
```

---

### 5. **GET /resumo-by-interval**
Retorna um resumo estatístico das vendas no intervalo de datas.

**Query Parameters:**
- `dtInit` (string, required): Data inicial no formato `YYYY-MM-DD`
- `dtFim` (string, required): Data final no formato `YYYY-MM-DD`

**Exemplo:**
```
GET /vendas/resumo-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
```

**Response:**
```json
{
  "total_vendas": 150,
  "total_valor": 45230.75,
  "total_desconto": 1250.50,
  "total_acrescimo": 320.00,
  "total_canceladas": 5
}
```

**Campos:**
- `total_vendas`: Total de vendas no período
- `total_valor`: Soma dos valores totais das vendas
- `total_desconto`: Soma dos descontos aplicados
- `total_acrescimo`: Soma dos acréscimos aplicados
- `total_canceladas`: Quantidade de vendas canceladas

---

## Estrutura das Entidades

### VendaEntity
Representa uma venda (NF-e/NFC-e).

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `id` | i64 | ID único da venda |
| `tip` | i32 | Tipo de documento |
| `mod` | i32 | Modelo do documento (65=NFC-e, 55=NF-e) |
| `serie_origin` | string | Série de origem |
| `serie` | string | Série do documento |
| `nr_nf_origin` | i32 | Número NF origem |
| `nr_nf` | i32 | Número da nota fiscal |
| `cnpj` | string | CNPJ do emitente |
| `doc_destinatario` | string? | CPF/CNPJ do destinatário |
| `dh_emi` | string | Data/hora de emissão |
| `dh_emi_canc` | string? | Data/hora de cancelamento |
| `total` | f64 | Valor total |
| `addition` | f64 | Valor de acréscimo |
| `discount` | f64 | Valor de desconto |
| `chave` | string | Chave de acesso NF-e |
| `chave_canc` | string? | Chave de cancelamento |
| `file_path` | string? | Caminho do arquivo XML |
| `cancel_file_path` | string? | Caminho do XML de cancelamento |
| `protocolo` | string? | Protocolo de autorização |
| `cancelled` | i32 | Status de cancelamento (0=ativa, 1=cancelada) |
| `created_at` | DateTime | Data de criação |
| `updated_at` | DateTime | Data de atualização |

### VendaItemEntity
Representa um item de venda.

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `id` | i64 | ID único do item |
| `venda_id` | i64 | ID da venda |
| `produto_code` | string | Código do produto |
| `produto_description` | string | Descrição do produto |
| `produto_medida` | string | Unidade de medida |
| `quantidade` | f64 | Quantidade (4 decimais) |
| `preco_unitario` | f64 | Preço unitário |
| `desconto` | f64 | Desconto do item |
| `desconto_rat` | f64 | Desconto rateado |
| `acrescimo` | f64 | Acréscimo do item |
| `acrescimo_rat` | f64 | Acréscimo rateado |
| `preco_total` | f64 | Preço total do item |
| `created_at` | DateTime | Data de criação |
| `updated_at` | DateTime | Data de atualização |

### VendaPagamentoEntity
Representa uma forma de pagamento utilizada na venda.

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `id` | i64 | ID único do pagamento |
| `venda_id` | i64 | ID da venda |
| `code` | string | Código da forma de pagamento (veja PaymentTypes) |
| `name` | string | Nome da forma de pagamento |
| `total_pagamento` | f64 | Valor pago |
| `created_at` | DateTime | Data de criação |
| `updated_at` | DateTime | Data de atualização |

### ResumeEntity
Representa um resumo de pagamentos agrupados por tipo.

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `id` | string (UUID) | ID único do resumo |
| `code` | PaymentTypes | Tipo de pagamento |
| `amount_s` | f64 | Valor total com sincronização |
| `amount_n` | f64 | Valor total sem sincronização |
| `updated_at` | i64 | Timestamp de atualização (millis) |
| `created_at` | i64 | Timestamp de criação (millis) |

---

## Códigos de Formas de Pagamento (Padrão NF-e)

| Código | PaymentTypes | Descrição |
|--------|--------------|-----------|
| 01 | Dinheiro | Dinheiro |
| 02 | Cheque | Cheque |
| 03 | CartaoDeCredito | Cartão de Crédito |
| 04 | CartaoDeDebito | Cartão de Débito |
| 05 | CreditoLoja | Crédito Loja |
| 10 | ValeAlimentacao | Vale Alimentação |
| 11 | ValeRefeicao | Vale Refeição |
| 12 | ValePresente | Vale Presente |
| 13 | ValeCombustivel | Vale Combustível |
| 14 | DuplicataMercantil | Duplicata Mercantil |
| 15 | BoletoBancario | Boleto Bancário |
| 90 | SemPagamento | Sem pagamento |
| 99 | Outros | Outros |

---

## Exemplos de Uso

### Buscar vendas de um dia específico
```bash
curl "http://localhost:8088/vendas/get-vendas-by-interval?dtInit=2024-06-15&dtFim=2024-06-15"
```

### Buscar itens vendidos no mês
```bash
curl "http://localhost:8088/vendas/get-items-by-interval?dtInit=2024-06-01&dtFim=2024-06-30"
```

### Buscar resumo anual
```bash
curl "http://localhost:8088/vendas/resumo-by-interval?dtInit=2024-01-01&dtFim=2024-12-31"
```

---

## Notas

1. **Formato de Data**: Todas as datas devem ser enviadas no formato `YYYY-MM-DD`
2. **CORS**: O servidor possui CORS habilitado para permitir requisições de qualquer origem
3. **Porta**: O servidor HTTP roda na porta `8088`
4. **Cancelamento**: Vendas canceladas têm `cancelled = 1` e mantêm os dados originais
5. **NF-e**: A estrutura segue o padrão de NF-e/NFC-e da SEFAZ
