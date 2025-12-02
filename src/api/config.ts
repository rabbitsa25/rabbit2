import { invoke } from "@tauri-apps/api/core";

// Types
export interface ConfigEntity {
  id: string;
  flow_base_url: string;
  code_uf: number;
  nserie_sat: string;
  nserie_sat_nao?: string;
  nr_nf_sim: number;
  nr_nf_nao: number;
  sign_ac?: string;
  regime_tributario: string;
  cnpj: string;
  name: string;
  short_name?: string;
  zipcode: string;
  address_name: string;
  address_number: string;
  address_city: string;
  address_city_code?: string;
  tipo_ambiente: string;
  address_cpl?: string;
  address_neiborhood: string;
  address_state: string;
  fone?: string;
  created_at: number;
  updated_at: number;
  percent_s: number;
  only_money: number;
  error_as_success: number;
  ie?: string;
  pagamentos?: string;
  ignore_cpf: number;
  numero_caixa: number;
  emitir_l: number;
  habilitar_contador: number;
  habilitar_contador_nao: number;
  controle_estoque: number;
  modelo: number;
}

export interface CreateOrUpdateConfigDto {
  id?: string;
  flow_base_url?: string;
  code_uf?: number;
  nserie_sat?: string;
  nserie_sat_nao?: string;
  nr_nf_sim?: number;
  nr_nf_nao?: number;
  sign_ac?: string;
  regime_tributario?: string;
  cnpj?: string;
  name?: string;
  short_name?: string;
  zipcode?: string;
  address_name?: string;
  address_number?: string;
  address_city?: string;
  address_city_code?: string;
  tipo_ambiente?: string;
  address_cpl?: string;
  address_neiborhood?: string;
  address_state?: string;
  fone?: string;
  percent_s?: number;
  only_money?: number;
  error_as_success?: number;
  ie?: string;
  pagamentos?: string;
  ignore_cpf?: number;
  numero_caixa?: number;
  emitir_l?: number;
  habilitar_contador?: number;
  habilitar_contador_nao?: number;
  controle_estoque?: number;
  modelo?: number;
}

export interface UpdatePercentConfigDto {
  percent_s: number;
}

export interface CnpjResponseDto {
  cnpj: string;
  razao_social?: string;
  nome_fantasia?: string;
  logradouro?: string;
  numero?: string;
  complemento?: string;
  bairro?: string;
  municipio?: string;
  uf?: string;
  cep?: string;
  telefone?: string;
  email?: string;
}

// API Functions (Controller-like)
export const ConfigAPI = {
  /**
   * GET /config/cnpj/:cnpj
   * Consulta informações de um CNPJ
   */
  getCnpj: async (cnpj: string): Promise<CnpjResponseDto> => {
    return await invoke<CnpjResponseDto>("get_cnpj", { cnpj });
  },

  /**
   * GET /config/
   * Obtém a primeira configuração (default)
   */
  getFirstConfig: async (): Promise<ConfigEntity | null> => {
    return await invoke<ConfigEntity | null>("get_first_config");
  },

  /**
   * POST /config/
   * Cria ou atualiza uma configuração
   */
  createOrUpdate: async (body: CreateOrUpdateConfigDto): Promise<ConfigEntity> => {
    return await invoke<ConfigEntity>("create_or_update_config", { body });
  },

  /**
   * PATCH /config/percent
   * Atualiza apenas o percentual de desconto
   */
  updatePercent: async (body: UpdatePercentConfigDto): Promise<ConfigEntity> => {
    return await invoke<ConfigEntity>("update_percent_config", { body });
  },

  // Legacy methods (mantidos para compatibilidade)
  getById: async (id: string): Promise<ConfigEntity | null> => {
    return await invoke<ConfigEntity | null>("get_config", { id });
  },

  save: async (config: ConfigEntity): Promise<ConfigEntity> => {
    return await invoke<ConfigEntity>("save_config", { config });
  },

  list: async (): Promise<ConfigEntity[]> => {
    return await invoke<ConfigEntity[]>("list_configs");
  },
};

// Helper para formatar CNPJ
export const formatCNPJ = (cnpj: string): string => {
  const cleaned = cnpj.replace(/\D/g, "");
  if (cleaned.length !== 14) return cnpj;
  return cleaned.replace(/^(\d{2})(\d{3})(\d{3})(\d{4})(\d{2})$/, "$1.$2.$3/$4-$5");
};

// Helper para limpar CNPJ
export const cleanCNPJ = (cnpj: string): string => {
  return cnpj.replace(/\D/g, "");
};
