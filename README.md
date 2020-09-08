# Gerador arquivo de arquitetura para o projeto Amnesia

Este repositório contém um programa que gera de forma interativa o arquivo de arquitetura para o [Projeto Amnesia](http://amnesia.lasdpc.icmc.usp.br) utilizado na matéria Arquitetura de Computadores III do curso de Ciência da Computação da Pontifícia Universidade Católica de Minas Gerais.

## Download

Os objetos executáveis podem ser baixados de ["Releases"](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/releases).

## Compilação

### Pré-requisito

1- Verifique se o cargo está instalado em seu computador, digitando o seguinte comando no terminal (no Windows, prompt de comando):
```
cargo --version
```

2- Verifique se o compilador de rust está instalado em seu computador, digitando o seguinte comando no terminal (no Windows, prompt de comando):
```
rustc --version
```

3- Caso o cargo ou o compilador de rust não esteja instalado, instale através do site da Linguagem Rust: https://www.rust-lang.org/tools/install.

### Compilando

1- Baixe o [arquivo zip](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/archive/master.zip) deste repositório git.

2- Descompacte o arquivo zip.

3- Abra o terminal (no Windows, prompt de comando) na pasta 'Gerador_arquivo_arquitetura_Amnesia_RustLang' e digite o seguinte comando:
```
cargo build --release
```

4- Para executar o programa depois de compilado, digite no terminal (no Windows, prompt de comando) o seguinte comando:
```
cargo run
```

## Licença

Este programa está licensiado sobre [Licença MIT](LICENSE).
