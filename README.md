[English Version](README.EN.md)

# Gerador arquivo de arquitetura para o projeto Amnesia
[![deepcode](https://www.deepcode.ai/api/gh/badge?key=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJwbGF0Zm9ybTEiOiJnaCIsIm93bmVyMSI6IkhlbnJpcXVlbWNjIiwicmVwbzEiOiJHZXJhZG9yX2FycXVpdm9fYXJxdWl0ZXR1cmFfYW1uZXNpYV9SdXN0TGFuZyIsImluY2x1ZGVMaW50IjpmYWxzZSwiYXV0aG9ySWQiOjIzNTQyLCJpYXQiOjE2MDIxMDg1NDl9.S2q04KC96xRG5HEA2PFQKdO0Lxs7RHVlbKstkIutK7Q)](https://www.deepcode.ai/app/gh/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/_/dashboard?utm_content=gh%2FHenriquemcc%2FGerador_arquivo_arquitetura_amnesia_RustLang)

Este repositório contém o código fonte de um programa que gera de forma interativa o arquivo de arquitetura para o [Projeto Amnesia](http://amnesia.lasdpc.icmc.usp.br), utilizado na matéria Arquitetura de Computadores III do curso de Ciência da Computação da Pontifícia Universidade Católica de Minas Gerais.

## Download

Os objetos executáveis podem ser baixados de ["Releases"](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/releases).

## Como compilar

### Pré-requisito

1- Verifique se o cargo (da Linguagem Rust) está instalado em seu computador, digitando o seguinte comando no terminal (no Windows, prompt de comando):
```
cargo --version
```

2- Verifique se o compilador da Linguagem Rust está instalado em seu computador, digitando o seguinte comando no terminal:
```
rustc --version
```

3- Caso o cargo ou o compilador de rust não esteja instalado, instale através do site da Linguagem Rust: https://www.rust-lang.org/tools/install.

### Compilando

1- Baixe o [arquivo zip](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/archive/main.zip) deste repositório git.

2- Descompacte o arquivo zip.

3- Abra o terminal (no Windows, prompt de comando) na pasta extraida deste arquivo zip e digite o seguinte comando:
```
cargo build --release
```

4 - O objeto binário executável estará localizado na pasta 'release' dentro da pasta 'target':
```
./target/release/
```
Com o nome gerador_arquitetura_amnesia (gerador_arquitetura_amnesia.exe, no Windows).

## Licença

Este programa está licensiado sobre [Licença MIT](LICENSE).