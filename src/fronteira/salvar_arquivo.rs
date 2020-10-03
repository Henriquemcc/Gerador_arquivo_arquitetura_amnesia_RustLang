/*
 * MIT License
 *
 * Copyright (c) 2020 Henrique Mendonça Castelar Campos
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::fs::File;
use std::io;
use std::io::Write;

use crate::arquitetura::configuracoes::Arquitetura;
use crate::fronteira::myio;

/// Salva interativamente o arquivo da arquitetura com o usuario.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser salva.
pub fn salvar_arquitetura_arquivo(arquitetura: &Arquitetura)
{
    //Obtendo do usuario o nome do arquivo
    let nome_arquivo = obter_nome_arquivo();

    //Criando este arquivo
    let arquivo = File::create(nome_arquivo);

    //Caso algum erro ocorra, exibindo mensagem de erro e saindo da funcao.
    if arquivo.is_err()
    {
        eprint!("{}", arquivo.unwrap_err());
        return;
    }

    //Convertendo arquivo do tipo Result para o tipo File
    let arquivo = arquivo.unwrap();

    //Gravando configuracoes do amnesia para o arquivo.
    arquitetura.to_file(arquivo);
}

/// Obtem o nome do arquivo a ser salvo interativamente com o usuario.
/// # Return
/// * String - Nome do arquivo.
fn obter_nome_arquivo() -> String
{
    let mut nome_arquivo = String::new();
    let mut repetir = true;
    while repetir
    {
        print!("Nome do arquivo:");
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().trim().to_string();

        if !entrada.is_ascii()
        {
            eprintln!("O nome do arquivo deve estar em ascii")
        } else if !entrada.trim().ends_with(".xml")
        {
            eprintln!("O nome do arquivo deve terminar com a extensão .xml");
        } else {
            nome_arquivo = entrada;
            repetir = false;
        }
    }

    //Retornando o nome do arquivo
    return nome_arquivo;
}