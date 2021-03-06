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

/// Define a struct do tipo CPU para a tag CPU, que representa a configuração do CPU.
pub struct CPU
{
    word_size: usize
}

/// Implementacao de metodos para a struct CPU.
impl CPU
{
    /// Obtem o valor do atributo word_size.
    /// # Return
    /// * usize - Valor do atributo word_size.
    pub fn get_word_size(&self) -> usize
    {
        return self.word_size;
    }

    /// Cria uma nova instancia da struct CPU.
    /// # Return
    /// * CPU - Nova instancia da struct CPU.
    pub fn new() -> CPU
    {
        CPU
        {
            word_size: 4
        }
    }

    /// Converte uma instancia da struct CPU em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct CPU.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "word size:";
        dados += &self.get_word_size().to_string();

        return dados;
    }

    /// Converte uma instancia da struct do tipo CPU em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += "\t<CPU>\n";
        dados_para_arquivo += "\t\t<wordSize>";
        dados_para_arquivo += &self.get_word_size().to_string();
        dados_para_arquivo += "</wordSize>\n";
        dados_para_arquivo += "\t</CPU>\n";

        return dados_para_arquivo;
    }
}