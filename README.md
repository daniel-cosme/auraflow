# OdontoSaaS - Sistema de Agendamento Odontológico

Um sistema SaaS completo de agendamento odontológico desenvolvido com Rust + Leptos como frontend SPA.

## 🚀 Tecnologias Utilizadas

- **Rust**: Linguagem de programação de alto desempenho e segurança
- **Leptos**: Framework full-stack para aplicações web em Rust
- **TailwindCSS**: Framework de estilos utilitário
- **Cargo**: Gerenciador de pacotes do Rust
- **Trunk**: Ferramenta de build para aplicações web em Rust

## 📋 Requisitos

- Rust (versão estável mais recente)
- Cargo
- Trunk

## 🔧 Instalação

1. Clone este repositório
2. Navegue até o diretório do projeto
3. Instale as dependências:

```bash
# Instale o Trunk se ainda não tiver instalado
cargo install trunk

# Instale as dependências do projeto
cargo check
```

## ▶️ Execução

Para rodar o projeto em modo de desenvolvimento:

```bash
trunk serve
```

O servidor iniciará na porta 8080 por padrão. Acesse `http://localhost:8080` para visualizar o sistema.

## 🏗️ Estrutura de Pastas

```
src/
  main.rs                 # Ponto de entrada da aplicação
  lib.rs                  # Tipos compartilhados e estado global
  app/
    mod.rs                # Módulo principal do aplicativo
    routes.rs             # Definição das rotas
    components.rs         # Componentes reutilizáveis
  features/
    auth/                 # Componentes relacionados à autenticação
    appointments/         # Componentes relacionados às consultas
    patients/             # Componentes relacionados aos pacientes
  services/
    api.rs                # Cliente HTTP centralizado
  utils.rs                # Funções utilitárias
static/                   # Arquivos estáticos (CSS, imagens, etc.)
```

## 📄 Funcionalidades Implementadas

- [x] Tela de login
- [x] Dashboard com métricas
- [x] Agenda de consultas (diária/semanal)
- [x] Cadastro de novas consultas
- [x] Lista de pacientes
- [x] Detalhes do paciente
- [x] Configurações do usuário
- [x] Design responsivo
- [x] Modo claro/escuro
- [x] Mock de dados para demonstração

## 🔐 Credenciais de Teste

Para testar a funcionalidade de login, utilize:

- Email: `admin@example.com`
- Senha: `password123`

## 🎨 Design

- Layout SaaS profissional e minimalista
- Sidebar fixa à esquerda
- Header superior
- Design responsivo mobile-first
- Paleta de cores clínica (tons neutros + azul)
- Componentes modernos e intuitivos
- Suporte a modo escuro

## 📊 Páginas Disponíveis

1. **Login**: Autenticação no sistema
2. **Dashboard**: Visão geral com métricas
3. **Agenda**: Visualização das consultas
4. **Nova Consulta**: Formulário para agendar consulta
5. **Pacientes**: Lista de pacientes cadastrados
6. **Detalhe do Paciente**: Informações completas do paciente
7. **Configurações**: Gerenciamento de perfil

## 🛠️ API Mockada

O sistema utiliza endpoints mockados para simular comunicação com backend:

- `POST /auth/login` - Autenticação
- `GET /appointments` - Listar consultas
- `POST /appointments` - Criar consulta
- `GET /patients` - Listar pacientes
- `GET /patients/:id` - Obter detalhes do paciente

## 📝 Notas de Desenvolvimento

Este é um frontend SPA completo que simula um sistema SaaS real. Embora não tenha um backend real implementado, a estrutura está preparada para integração com APIs reais.

As funções assíncronas utilizam `async/await` do Rust com `spawn_local` para operações de rede simuladas.

## 🤝 Contribuições

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou enviar pull requests.