# Payment Plan

Calculadora de plano de pagamento da [Parcelado Lara](https://parceladolara.com.br).

Este projeto serve como base para o desenvolvimento de SDKs em diferentes linguagens, incluindo Node.js, WASM Web, Go, PHP e Python.

## Estrutura do Projeto

O projeto é dividido em pacotes, cada um com sua própria funcionalidade:

- **`cli`**: Código para a CLI (Command Line Interface) que permite calcular planos de pagamento via terminal _(Deprecated)_
- **`core`**: Lógica principal do cálculo dos planos de pagamento
- **`docs`**: Documentação detalhada do projeto
- **`generators`**: Geradores de código para diferentes linguagens
- **`sdks`**: SDKs (Software Development Kits) para diferentes linguagens (Node.js, Go, PHP, etc.)
- **`setup`**: Scripts de configuração para facilitar a execução do projeto
- **`uniffi-bindgen`**: Binário simples para gerar bindings de Rust para outras linguagens usando [UniFFI](https://github.com/mozilla/uniffi-rs)

## Instalação e Configuração

Para facilitar as configurações necessárias, você pode usar os scripts de setup que instalarão todas as dependências e configurarão o ambiente automaticamente.

### Setup Automático

**Arch Linux:**

```bash
chmod +x setup/arch.sh
./setup/arch.sh
```

**Debian/Ubuntu:**

```bash
chmod +x setup/debian.sh
./setup/debian.sh
```

### Instalação Manual

Caso você esteja em outra distribuição ou sistema operacional, será necessário instalar as seguintes dependências:

- [Rust](https://www.rust-lang.org/tools/install) (v1.81.0 ou superior)
- [PHP](https://www.php.net/downloads) (v8.1 ou superior, com FFI habilitado)
- [Node.js](https://nodejs.org/en/download/) (v22 ou superior)
- [Go](https://go.dev/doc/install) (v1.24.1 ou superior)
- [Python](https://www.python.org/downloads/) (v3.10 ou superior)
- [wasm-pack](https://rustwasm.github.io/) (v0.13.1)
- [Protocol Buffers](https://protobuf.dev/) (v3.21.12 ou superior)
- [protoc-gen-go](https://github.com/protocolbuffers/protobuf-go)
- Para sistemas Linux: ferramentas para compilação cross-platform (Windows)

## Testando o Projeto

Após instalar as dependências, você pode testar o projeto executando o seguinte comando na raiz do projeto:

```bash
make test
```

## Compilação por Linguagem

Cada SDK possui seu próprio comando de compilação no `Makefile`. Para compilar o projeto para uma linguagem específica:

```bash
make build-<linguagem>-sdk
```

**Exemplos:**

```bash
make build-kotlin-sdk    # Gera bindings Kotlin (requer dependências já instaladas)
make build-go-sdk        # Compila SDK Go
make build-python-sdk    # Compila SDK Python
make build-node-sdk      # Compila SDK Node.js
make build-php-sdk       # Compila SDK PHP
```

**💡 Para Kotlin:** Recomendamos usar `cd sdks/kotlin && ./setup.sh` para novos usuários, pois verifica e instala dependências automaticamente.

Para compilação no Windows:

```bash
make build-<linguagem>-sdk-windows
```

> **Nota:** Os comandos make assumem que você está executando em um sistema Unix-like.

## Linguagens Não Disponíveis

Caso você esteja integrando com a Lara e não encontrou um SDK na linguagem desejada:

1. **Se a linguagem estiver no ROADMAP** Estamos trabalhando nela. Você pode acompanhar o progresso na seção [Roadmap](#roadmap) abaixo.

2. **Se não estiver na lista:** Você pode usar os bindings em C no pacote `generators/c-bind` utilizando o [Foreign Function Interface (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface) da sua linguagem.

### Usando Bindings C

O SDK de PHP é um exemplo de como fazer essa integração usando FFI para chamar funções do Rust.

Para compilar o projeto para a ABI de C:

```bash
make build-c-bind
```

Os arquivos de cabeçalho e bibliotecas estarão disponíveis na pasta `lara-c-bind`.

### ⚠️ Importante sobre FFI

Trabalhar com FFI pode ser complexo. É recomendado ter conhecimento prévio sobre:

- Como funciona o FFI na linguagem desejada
- Alocação e liberação de memória
- Tipos de dados e seu gerenciamento na memória

O uso inadequado do FFI pode causar:

- Vazamentos de memória
- Corrupção de memória
- Outros problemas difíceis de depurar

## Roadmap

### SDKs Disponíveis e Em Desenvolvimento

- [x] **Node.js** - Disponível
- [x] **Go** - Disponível
- [x] **Python** - Disponível _(falta publicar no PyPI)_
- [x] **PHP** - Disponível
- [x] **WASM Web** - Disponível
- [x] **Kotlin** - Disponível
- [ ] **Swift** - Planejado
- [ ] **C#** - Planejado
