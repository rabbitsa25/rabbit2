import { invoke } from '@tauri-apps/api/core';

export interface Product {
    id?: number;
    code: string;
    name: string;
    active: number;
    balance: number;
    created_at: string;
    updated_at: string;
}

/**
 * API de Produtos - Ponte com Rust (Tauri Commands)
 * Corresponde ao ProdutosController do NestJS
 */
export class ProductsApi {
    /**
     * POST /products - Cria um novo produto
     */
    static async create(code: string, name: string): Promise<Product> {
        return await invoke<Product>('create_product', { code, name });
    }

    /**
     * GET /products/:id - Busca produto por ID
     */
    static async getById(id: number): Promise<Product | null> {
        return await invoke<Product | null>('get_product', { id });
    }

    /**
     * GET /products/code/:code - Busca produto por código
     */
    static async getByCode(code: string): Promise<Product | null> {
        return await invoke<Product | null>('get_product_by_code', { code });
    }

    /**
     * GET /products - Lista todos os produtos
     */
    static async getAll(): Promise<Product[]> {
        return await invoke<Product[]>('get_all_products');
    }

    /**
     * Lista apenas produtos ativos
     */
    static async getAllActive(): Promise<Product[]> {
        return await invoke<Product[]>('list_active_products');
    }

    /**
     * PUT /products/:id - Atualiza um produto
     */
    static async update(
        id: number,
        data: {
            code?: string;
            name?: string;
            active?: number;
            balance?: number;
        }
    ): Promise<Product> {
        return await invoke<Product>('update_product', {
            id,
            code: data.code ?? null,
            name: data.name ?? null,
            active: data.active ?? null,
            balance: data.balance ?? null,
        });
    }

    /**
     * DELETE /products/:id - Deleta um produto (soft delete)
     */
    static async delete(id: number): Promise<void> {
        await invoke('delete_product', { id });
    }

    /**
     * PATCH /products/:id/increment - Incrementa saldo
     */
    static async incrementBalance(id: number, amount: number): Promise<Product> {
        return await invoke<Product>('increment_product_balance', { id, amount });
    }

    /**
     * PATCH /products/:id/decrement - Decrementa saldo
     */
    static async decrementBalance(id: number, amount: number): Promise<Product> {
        return await invoke<Product>('decrement_product_balance', { id, amount });
    }
}
