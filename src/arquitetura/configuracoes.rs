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
use std::io::Write;

use crate::arquitetura::cache::HaveCache;
use crate::arquitetura::cpu::CPU;
use crate::arquitetura::main_memory::MainMemory;
use crate::arquitetura::processor::Processor;
use crate::arquitetura::trace::Trace;
use crate::arquitetura::virtual_memory::HaveVirtualMemory;

/// Define a struct do tipo Arquitetura para armazenar as configur configuracoes da arquitetura do amnesia.
pub struct Arquitetura
{
    pub processor: Processor,
    pub trace: Trace,
    pub cpu: CPU,
    pub cache: HaveCache,
    pub main_memory: MainMemory,
    pub virtual_memory: HaveVirtualMemory,
}

/// Implementacao de metodos para a struct Arquitetura.
impl Arquitetura
{
    /// Cria uma nova instancia da struct Arquitetura
    /// # Return
    /// * Arquitetura - Nova instancia da struct Arquitetura.
    pub fn new() -> Arquitetura
    {
        Arquitetura
        {
            processor: Processor::new(),
            trace: Trace::new(),
            cpu: CPU::new(),
            cache: HaveCache::No,
            main_memory: MainMemory::new(),
            virtual_memory: HaveVirtualMemory::No,
        }
    }

    ///Escreve a Arquitetura do amnesia em um arquivo.
    /// # Arguments
    /// * arquivo - Arquivo no qual os dados da arquitetura serao escritos.
    pub fn to_file(&self, mut arquivo: File)
    {
        //Gerando a string
        let mut dados_para_arquivo = String::new();

        //Gerando o cabecalho
        dados_para_arquivo += "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n";
        dados_para_arquivo += "\n";
        dados_para_arquivo += "<!DOCTYPE AmnesiaConfiguration SYSTEM \"Configuration/amnesia.dtd\">\n";
        dados_para_arquivo += "<?xml-stylesheet type=\"text/css\" href=\"teste.css\"?>\n";
        dados_para_arquivo += "<AmnesiaConfiguration>\n";

        //Gerando as informacoes do processador
        dados_para_arquivo += &self.processor.to_string_arquivo();

        //Gerando as informacoes do trace
        dados_para_arquivo += &self.trace.to_string_arquivo();

        //Gerando as informacoes do cpu
        dados_para_arquivo += &self.cpu.to_string_arquivo();

        //Gerando as informacoes do memoria ram
        dados_para_arquivo += &self.main_memory.to_string_arquivo();

        //Gerando as informacoes da cache
        dados_para_arquivo += &self.cache.to_string_arquivo();

        //Gerando as informacoes da memoria virtual
        dados_para_arquivo += &self.virtual_memory.to_string_arquivo();

        //Imprimindo o cabecalho final
        dados_para_arquivo += "</AmnesiaConfiguration>";


        //Escrevendo dados no arquivo
        let sucesso = arquivo.write(dados_para_arquivo.as_bytes());

        if sucesso.is_err()
        {
            eprint!("{}", sucesso.unwrap_err());
        }

        //Fechando arquivo
        drop(arquivo);
    }

    /// Converte uma struct Arquitetura em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct Arquitetura.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "Processor:";
        dados += "\n";
        dados += &self.processor.to_string();
        dados += "\n\n";

        dados += "Trace:";
        dados += "\n";
        dados += &self.trace.to_string();
        dados += "\n\n";

        dados += "CPU:";
        dados += "\n";
        dados += &self.cpu.to_string();
        dados += "\n\n";

        match self.cache
        {
            HaveCache::Yes(cache_struct) =>
                {
                    dados += "Cache:";
                    dados += "\n";
                    dados += &cache_struct.to_string();
                    dados += "\n\n";
                }
            _ => {}
        }

        dados += "Main Memory:";
        dados += "\n";
        dados += &self.main_memory.to_string();
        dados += "\n\n";

        match &self.virtual_memory
        {
            HaveVirtualMemory::Yes(virtual_memory_struct) =>
                {
                    dados += "Virtual Memory:";
                    dados += "\n";
                    dados += &virtual_memory_struct.to_string();
                }
            _ => {}
        }

        return dados;
    }
}