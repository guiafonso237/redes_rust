<h1 align="center">
  🦀 RustScan CLI - Network Subnet Scanner
</h1>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt="Linux" />
  <img src="https://img.shields.io/badge/CLI-Terminal-4D4D4D?style=for-the-badge" alt="CLI" />
</p>

<p align="center">
  Uma ferramenta de linha de comando (CLI) leve e resiliente para varredura de portas TCP em redes locais, construída inteiramente em Rust para explorar a interação direta com o sistema operacional.
</p>

## 🧠 Arquitetura e Desafio Técnico

Este projeto atua na interseção entre o desenvolvimento de software e a infraestrutura de redes. O maior desafio ao construir ferramentas de rede em baixo nível é a **imprevisibilidade**.

Em vez de depender de frameworks pesados, a aplicação foi desenhada para interagir diretamente com a biblioteca padrão (`std::net`). 
- **Resiliência em Sockets:** A varredura gerencia timeouts de chamadas de sistema (syscalls) rigorosamente via `TcpStream::connect_timeout`.
- **Tratamento de Erros Silencioso:** Em redes, falhas de conexão são a regra, não a exceção. O projeto utiliza o sistema de *Pattern Matching* do Rust (`match Ok/Err`) para descartar falhas de rotas e portas bloqueadas por firewalls sem gerar _panics_ ou overhead no processo.
- **Topologia Dinâmica:** O algoritmo descobre a interface de rede ativa da máquina hospedeira e calcula dinamicamente a sub-rede `/24` (ex: `192.168.1.X`), permitindo execução plug-and-play em qualquer ambiente local.

---

## 🚀 Funcionalidades

* **Descoberta Dinâmica de Sub-rede (`-g` ou `--mostrar-faixa-ip`):** Identifica automaticamente o IP local da máquina e mapeia a faixa de rede (ex: `192.168.1.X`) sem necessidade de input manual.
* **Varredura Específica por Intervalo (`-p` ou `--porta`):** Permite escanear um range customizado de portas (ex: 80 até 443) em todos os 254 hosts da rede local.
* **Varredura Completa Intensiva (`-a` ou `--mostrar-todos`):** Opção para realizar uma busca profunda nas 10.000 portas mais utilizadas em todos os dispositivos conectados.
* **Interface CLI Robusta:** Construída com a biblioteca `clap`, oferecendo argumentos e flags estruturados no padrão Unix.

---

## 💻 Instalação e Uso

### Pré-requisitos
- [Rust toolchain](https://www.rust-lang.org/tools/install) instalado.

### Instalação
```bash
git clone [https://github.com/guiafonso237/redes_rust.git]
cd NOME_DO_REPOSITORIO
cargo build --release