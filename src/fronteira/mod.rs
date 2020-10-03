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

pub mod criar_arquitetura;
pub mod alterar_arquitetura;
pub mod exibir_arquitetura;
pub mod salvar_arquivo;
pub(crate) mod myio;


pub fn bem_vindo()
{
    //Exibindo mensagem de bem vindo
    println!("Bem vindo ao gerador de arquitetura para o Amnesia.");
    println!("Este programa serve para criar um arquivo de arquitetura para o simulador Amnesia.");
    println!("O Amnesia pode ser baixado em: http://amnesia.lasdpc.icmc.usp.br");
}

pub fn obter_comando() -> u8
{
    let mut comando: u8;
    loop
    {
        //Exibindo os comandos
        println!("O que deseja fazer?");
        println!("0 - Sair.");
        println!("1 - Criar nova configuração de arquitetura para o Amnesia.");
        println!("2 - Exibir atual configuração de arquitetura do Amnesia.");
        println!("3 - Alterar atual configuração de arquitetura do Amnesia.");
        println!("4 - Salvar configuração de arquitetura em um arquivo.");

        //Obtendo o comando
        comando = myio::read_u8();

        if comando > 4
        {
            eprintln!("Comando inválido!");
        } else {
            break;
        }
    }
    return comando;
}