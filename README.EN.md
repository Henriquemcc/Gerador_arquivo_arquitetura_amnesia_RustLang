[Versão em Português](README.md)

# Generator of architecture file for the Amnesia project
[![deepcode](https://www.deepcode.ai/api/gh/badge?key=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJwbGF0Zm9ybTEiOiJnaCIsIm93bmVyMSI6IkhlbnJpcXVlbWNjIiwicmVwbzEiOiJHZXJhZG9yX2FycXVpdm9fYXJxdWl0ZXR1cmFfYW1uZXNpYV9SdXN0TGFuZyIsImluY2x1ZGVMaW50IjpmYWxzZSwiYXV0aG9ySWQiOjIzNTQyLCJpYXQiOjE2MDIxMDg1NDl9.S2q04KC96xRG5HEA2PFQKdO0Lxs7RHVlbKstkIutK7Q)](https://www.deepcode.ai/app/gh/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/_/dashboard?utm_content=gh%2FHenriquemcc%2FGerador_arquivo_arquitetura_amnesia_RustLang)

This repository contains the source code of a program that interactively generates the architecture file for the [Amnesia project](http://amnesia.lasdpc.icmc.usp.br), which was used in the subject Computer Architecture III of the Computer Science course at the Pontifical Catholic University of Minas Gerais.

## Download

The executable object can be downloaded from ["Releases"](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/releases).

## How to compile

### Requirements

1- Verify if RustLang's Cargo is installed on your computer, typing the following command on the terminal (on Windows, is the command prompt):
```
cargo --version
```

2- Verify if RustLang's compiler is installer on your computer, typing the following command on the terminal:
```
rustc --version
```

3- If RustLang's Cargo or RustLang's compiler is not installed, install through the RustLang website: https://www.rust-lang.org/tools/install.

### Compiling

1- Download this git repository's [zip file](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/archive/main.zip).

2- Unzip the zip file.

3- Open the terminal on the extracted folder from this zip file and type the following command:
```
cargo build --release
```

4- The binary executable object will be located on the 'release' folder inside the 'target' folder:
```
./target/release/
```
With the file name gerador_arquitetura_amnesia (gerador_arquitetura_amnesia.exe, on Windows).

## License

This program is licensed under [MIT License](LICENSE).