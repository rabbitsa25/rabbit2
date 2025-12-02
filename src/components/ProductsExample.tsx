import { useState, useEffect } from 'react';
import { ProductsApi, Product } from '../api/products';

/**
 * Exemplo de componente React usando a API de Produtos
 */
export function ProductsExample() {
    const [products, setProducts] = useState<Product[]>([]);
    const [loading, setLoading] = useState(false);
    const [code, setCode] = useState('');
    const [name, setName] = useState('');

    // Carrega produtos ao montar o componente
    useEffect(() => {
        loadProducts();
    }, []);

    const loadProducts = async () => {
        try {
            setLoading(true);
            const data = await ProductsApi.getAll();
            setProducts(data);
        } catch (error) {
            console.error('Erro ao carregar produtos:', error);
        } finally {
            setLoading(false);
        }
    };

    const handleCreate = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            await ProductsApi.create(code, name);
            setCode('');
            setName('');
            loadProducts();
        } catch (error) {
            console.error('Erro ao criar produto:', error);
        }
    };

    const handleDelete = async (id: number) => {
        try {
            await ProductsApi.delete(id);
            loadProducts();
        } catch (error) {
            console.error('Erro ao deletar produto:', error);
        }
    };

    const handleIncrement = async (id: number, amount: number) => {
        try {
            await ProductsApi.incrementBalance(id, amount);
            loadProducts();
        } catch (error) {
            console.error('Erro ao incrementar saldo:', error);
        }
    };

    const handleDecrement = async (id: number, amount: number) => {
        try {
            await ProductsApi.decrementBalance(id, amount);
            loadProducts();
        } catch (error) {
            console.error('Erro ao decrementar saldo:', error);
        }
    };

    const handleUpdate = async (id: number, updates: Partial<Product>) => {
        try {
            await ProductsApi.update(id, updates);
            loadProducts();
        } catch (error) {
            console.error('Erro ao atualizar produto:', error);
        }
    };

    if (loading) return <div>Carregando...</div>;

    return (
        <div>
            <h2>Gerenciamento de Produtos</h2>

            {/* Formulário de criação */}
            <form onSubmit={handleCreate}>
                <input
                    type="text"
                    placeholder="Código"
                    value={code}
                    onChange={(e) => setCode(e.target.value)}
                    required
                />
                <input
                    type="text"
                    placeholder="Nome"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                />
                <button type="submit">Criar Produto</button>
            </form>

            {/* Lista de produtos */}
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Código</th>
                        <th>Nome</th>
                        <th>Saldo</th>
                        <th>Status</th>
                        <th>Ações</th>
                    </tr>
                </thead>
                <tbody>
                    {products.map((product) => (
                        <tr key={product.id}>
                            <td>{product.id}</td>
                            <td>{product.code}</td>
                            <td>{product.name}</td>
                            <td>{product.balance.toFixed(4)}</td>
                            <td>{product.active === 1 ? 'Ativo' : 'Inativo'}</td>
                            <td>
                                <button onClick={() => handleIncrement(product.id!, 10)}>
                                    +10
                                </button>
                                <button onClick={() => handleDecrement(product.id!, 5)}>
                                    -5
                                </button>
                                <button onClick={() => handleUpdate(product.id!, { name: 'Novo Nome' })}>
                                    Renomear
                                </button>
                                <button onClick={() => handleDelete(product.id!)}>
                                    Deletar
                                </button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}
