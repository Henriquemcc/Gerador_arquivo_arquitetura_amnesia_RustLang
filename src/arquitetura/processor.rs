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

/// Define a struct do tipo Processor.
pub struct Processor
{
    processor_contains: usize,
    create_trace_file: usize,
}

/// Implementacao de metodos para a struct do tipo Processor.
impl Processor
{
    /// Altera o valor do atributo processor_contains.
    /// # Arguments
    /// * processor_contains - Novo valor para o atributo processor_contains.
    pub fn set_processor_contains(&mut self, processor_contains: usize)
    {
        self.processor_contains = processor_contains;
    }

    /// Obtem o valor do atributo processor_contains.
    /// # Return
    /// * usize - Valor do atributo processor_contains.
    pub fn get_processor_contains(&self) -> usize
    {
        return self.processor_contains;
    }

    /// Altera o valor do atributo create_trace_file.
    /// # Arguments
    /// * create_trace_file - Novo valor para o atributo create_trace_file.
    pub fn set_create_trace_file(&mut self, create_trace_file: usize)
    {
        self.create_trace_file = create_trace_file;
    }

    /// Obtem o valor do atributo create_trace_file.
    /// # Return
    /// * usize - Valor do atributo create_trace_file.
    pub fn get_create_trace_file(&self) -> usize
    {
        return self.create_trace_file;
    }

    /// Cria uma nova instancia da struct Processor.
    /// # Return
    /// * Processor - Nova instancia da struct Processor.
    pub fn new() -> Processor
    {
        Processor
        {
            processor_contains: 0,
            create_trace_file: 0,
        }
    }

    /// Converte uma instancia da struct Processor em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct Processor.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "processor contains:";
        dados += &self.get_processor_contains().to_string();
        dados += "\n";
        dados += "create trace file:";
        dados += &self.get_create_trace_file().to_string();

        return dados;
    }

    /// Converte uma struct do tipo Processor em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += "\t<Processor>\n";
        dados_para_arquivo += "\t\t<processorContains>";
        dados_para_arquivo += &self.get_processor_contains().to_string();
        dados_para_arquivo += "</processorContains>\n";
        dados_para_arquivo += "\t\t<createTraceFile>";
        dados_para_arquivo += &self.get_create_trace_file().to_string();
        dados_para_arquivo += "</createTraceFile>\n";
        dados_para_arquivo += "\t</Processor>\n";

        return dados_para_arquivo;
    }
}