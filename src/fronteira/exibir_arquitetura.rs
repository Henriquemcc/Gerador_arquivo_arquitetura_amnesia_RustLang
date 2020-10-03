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

use crate::arquitetura::configuracoes::Arquitetura;
use crate::fronteira::myio;

/// Exibe a arquitetura atual para o Amnesia.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser exibida.
pub fn exibir_arquitetura(arquitetura: &Arquitetura)
{
    let exibir_tudo = obter_modo_de_exibir_arquitetura();

    if exibir_tudo == 1
    {
        println!();
        println!("{}", arquitetura.to_string());
        println!();
    } else {
        let configuracao_a_ser_exibida = obter_parte_da_arquitetura_a_ser_exibinda();

        println!();

        if configuracao_a_ser_exibida == 0
        {
            println!("Processor:");
            println!("{}", arquitetura.processor.to_string());
        } else if configuracao_a_ser_exibida == 1
        {
            println!("Trace:");
            println!("{}", arquitetura.trace.to_string());
        } else if configuracao_a_ser_exibida == 2
        {
            println!("CPU:");
            println!("{}", arquitetura.cpu.to_string());
        } else if configuracao_a_ser_exibida == 3
        {
            println!("Cache:");
            println!("{}", arquitetura.cache.to_string());
        } else if configuracao_a_ser_exibida == 4
        {
            println!("Main Memory:");
            println!("{}", arquitetura.main_memory.to_string());
        } else if configuracao_a_ser_exibida == 5
        {
            println!("virtual Memory:");
            println!("{}", arquitetura.virtual_memory.to_string());
        }

        println!();
    }
}

/// Obtem qual parte da arquitetura o usuario deseja que seja exibida.
/// # Return
/// * u8 - Parte da arquitetura que o usuario que que seja exibida.
fn obter_parte_da_arquitetura_a_ser_exibinda() -> u8
{
    let mut configuracao: u8 = 0;

    let mut repetir = true;
    while repetir
    {
        println!("Qual configuração deseja que seja exibida?");
        println!("0 - processor");
        println!("1 - trace");
        println!("2 - cpu");
        println!("3 - cache");
        println!("4 - main_memory");
        println!("5 - virtual_memory");

        let entrada = myio::read_u8();

        if entrada > 5
        {
            eprintln!("Entrada inválida!");
        } else {
            configuracao = entrada;
            repetir = false;
        }
    }

    return configuracao;
}

/// Obtem o modo que o usuario deseja para visualizae a arquitetura atual do Amnesia.
/// # Return
/// * u8 - Modo de exibicao da arquitetura do Amnesia.
fn obter_modo_de_exibir_arquitetura() -> u8
{
    let mut comando: u8 = 0;

    let mut repetir = true;
    while repetir
    {
        //Exibindo os comandos
        println!("Deseja exibir toda as configurações?");
        println!("1 - Sim.");
        println!("0 - Não");
        let entrada = myio::read_u8();

        if entrada > 1
        {
            eprintln!("Entrada inválida!");
        } else {
            comando = entrada;
            repetir = false;
        }
    }


    return comando;
}