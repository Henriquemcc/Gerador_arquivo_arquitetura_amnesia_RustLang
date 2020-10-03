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

/// Define uma struct do tipo MainMemory para a tag MainMemory, que representa as configurações da memória principal da arquitetura do Amnesia.
pub struct MainMemory
{
    block_size: usize,
    memory_size: usize,
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
}

/// Implementacao de metodos para a struct MainMemory.
impl MainMemory
{
    /// Altera o valor do atributo block_size.
    /// # Arguments
    /// * block_size - Novo valor para o atributo block_size.
    pub fn set_block_size(&mut self, block_size: usize)
    {
        self.block_size = block_size;
    }

    /// Obtem o valor do atributo block_size.
    /// # Return
    /// * usize - Valor do atributo block_size.
    pub fn get_block_size(&self) -> usize
    {
        return self.block_size;
    }

    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Arguments
    /// * usize - Valor do atributo memory_size.
    pub fn get_memory_size(&self) -> usize
    {
        return self.memory_size;
    }

    /// Altera o valor do atributo cicles_per_access.
    /// # Arguments
    /// * cicles_per_access - Novo valor para o atributo cicles_per_access.
    pub fn set_cicles_per_access(&mut self, cicles_per_access: usize)
    {
        self.cicles_per_access = cicles_per_access;
    }

    /// Obtem o valor do atributo cicles_per_access.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access.
    pub fn get_cicles_per_access(&self) -> usize
    {
        return self.cicles_per_access;
    }

    /// Altera o valor do atributo cicles_per_access_read.
    /// # Arguments
    /// * cicles_per_access_read - Novo valor para o atributo cicles_per_access_read.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read: usize)
    {
        self.cicles_per_access_read = cicles_per_access_read;
    }

    /// Obtem o valor do atributo cicles_per_access_read.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_read.
    pub fn get_cicles_per_access_read(&self) -> usize
    {
        return self.cicles_per_access_read;
    }

    /// Altera o valor do atributo cicles_per_access_wirte.
    /// # Arguments
    /// * cicles_per_access_write - Novo valor para o atributo cicles_per_access_write.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write: usize)
    {
        self.cicles_per_access_write = cicles_per_access_write;
    }

    /// Obtem o valor do atributo cicles_per_access_write.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_write.
    pub fn get_cicles_per_access_write(&self) -> usize
    {
        return self.cicles_per_access_write;
    }

    /// Altera o valor do atributo time_cicle.
    /// # Arguments
    /// * time_cicle - Novo valor para o atributo time_cicle.
    pub fn set_time_cicle(&mut self, time_cicle: usize)
    {
        self.time_cicle = time_cicle;
    }

    /// Obtem o valor do atributo time_cicle.
    /// # Return
    /// * usize - Valor do atributo time_cicle.
    pub fn get_time_cicle(&self) -> usize
    {
        return self.time_cicle;
    }

    /// Cria uma nova instancia da struct MainMemory.
    /// # Return
    /// * MainMemory - Nova instancia da struct MainMemory.
    pub fn new() -> MainMemory
    {
        MainMemory
        {
            block_size: 0,
            memory_size: 0,
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
        }
    }

    /// Converte uma struct do tipo MainMemory em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct MainMemory
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "block size:";
        dados += &self.get_block_size().to_string();
        dados += "\n";

        dados += "memory size:";
        dados += &self.get_memory_size().to_string();
        dados += "\n";

        dados += "cicles per access:";
        dados += &self.get_cicles_per_access().to_string();
        dados += "\n";

        dados += "cicles per access read:";
        dados += &self.get_cicles_per_access_read().to_string();
        dados += "\n";

        dados += "cicles per access write:";
        dados += &self.get_cicles_per_access_write().to_string();
        dados += "\n";

        dados += "time cicle:";
        dados += &self.get_time_cicle().to_string();

        return dados;
    }

    /// Converte uma struct do tipo MainMemory em uma String sendo esta parte do conteudo de um arquivo XML.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        dados_para_arquivo += "\t<MainMemory>\n";
        dados_para_arquivo += "\t\t<blockSize>";
        dados_para_arquivo += &self.get_block_size().to_string();
        dados_para_arquivo += "</blockSize>\n";
        dados_para_arquivo += "\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "</memorySize>\n";
        dados_para_arquivo += "\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";
        dados_para_arquivo += "\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";
        dados_para_arquivo += "\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";
        dados_para_arquivo += "\t</MainMemory>\n";

        return dados_para_arquivo;
    }
}