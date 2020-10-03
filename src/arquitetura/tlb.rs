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

use crate::arquitetura::replacement_algorithm::ReplacementAlgorithm;

/// Define o tipo de TLB.
#[derive(Copy, Clone, PartialEq)]
pub enum TLBType
{
    None,
    Unified(UnifiedTLB),
    Split(SplitTLB),
}

/// Implementacao de metodos para a enum TLBType.
impl TLBType
{
    /// Converte uma enum do tipo TLBType em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da enum TLBType.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        match self
        {
            TLBType::Split(split_tlb_struct) => dados += &split_tlb_struct.to_string(),
            TLBType::Unified(unified_tlb_struct) => dados += &unified_tlb_struct.to_string(),
            _ => {}
        }

        return dados;
    }

    /// Converte uma enum do tipo TLBType em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        match self
        {
            TLBType::Split(split_tlb_struct) => dados_para_arquivo += &split_tlb_struct.to_string_arquivo(),
            TLBType::Unified(unifiesd_tlb_struct) => dados_para_arquivo += &unifiesd_tlb_struct.to_string_arquivo(),
            _ => {}
        }

        return dados_para_arquivo;
    }
}

/// Define uma struct do tipo UnifiedTLB
#[derive(Copy, Clone, PartialEq)]
pub struct UnifiedTLB
{
    memory_size: usize,
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    replacement_algorithm: ReplacementAlgorithm,
}

/// Implementacao dos metodos da struct UnifiedTLB.
impl UnifiedTLB
{
    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Return
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

    /// Altera o valor do atributo cicles_per_access_write.
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

    /// Altera o valor do atributo replacement_algorithm.
    /// # Arguments
    /// * replacement_algorithm - Novo valor para o atributo replacement_algorithm.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm)
    {
        self.replacement_algorithm = replacement_algorithm;
    }

    /// Obtem o valor do atributo replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo replacement_algorithm.
    pub fn get_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.replacement_algorithm;
    }

    /// Converte uma struct do tipo UnifiedTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo os atributos da struct UnifiedTLB.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "unified";
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

        dados += "time cicle unified tlb:";
        dados += &self.get_time_cicle().to_string();
        dados += "\n";

        dados += "replacement algorithm:";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }

        return dados;
    }

    /// Converte uma struct do tipo UnifiedTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        dados_para_arquivo += "\t\t<unifiedTLB>\n";

        dados_para_arquivo += "\t\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "<memorySize>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccess>";
        dados_para_arquivo += &self.get_cicles_per_access().to_string();
        dados_para_arquivo += "</ciclesPerAccess>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t\t<replacementAlgorithm>";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</replacementAlgorithm>\n";

        dados_para_arquivo += "\t\t</unifiedTLB>\n";

        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct UnifiedTLB.
    /// # Return
    /// * UnifiedTLB - Nova instancia da struct UnifiedTLB.
    pub fn new() -> UnifiedTLB
    {
        UnifiedTLB
        {
            memory_size: 0,
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            replacement_algorithm: ReplacementAlgorithm::Null,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct SplitTLB
{
    instruction_tlb: InstructionTLB,
    data_tlb: DataTLB,
}

impl SplitTLB
{
    /// Altera o valor do atributo instruction_tlb.
    /// # Arguments
    /// * instruction_tlb - Novo valor para o atributo instruction_tlb.
    pub fn set_instruction_tlb(&mut self, instruction_tlb: InstructionTLB)
    {
        self.instruction_tlb = instruction_tlb;
    }

    /// Obtem o valor do atributo instruction_tlb.
    /// # Return
    /// * InstructionTLB - Valor do atributo instruction_tlb.
    pub fn get_instruction_tlb(&self) -> InstructionTLB
    {
        return self.instruction_tlb;
    }

    /// Altera o valor do atributo data_tlb.
    /// # Arguments
    /// * data_tlb - Novo valor para o atributo data_tlb.
    pub fn set_data_tlb(&mut self, data_tlb: DataTLB)
    {
        self.data_tlb = data_tlb;
    }

    /// Obtem o valor do atributo data_tlb.
    /// # Return
    /// * DataTLB - Valor do atributo data_tlb.
    pub fn get_data_tlb(&self) -> DataTLB
    {
        return self.data_tlb;
    }

    /// Converte uma struct do tipo SplitTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo os atributos da enum SplitTLB.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();
        dados += &self.get_instruction_tlb().to_string();
        dados += &self.get_data_tlb().to_string();
        return dados;
    }

    /// Converte uma struct do tipo SplitTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += &self.get_instruction_tlb().to_string_arquivo();
        dados_para_arquivo += &self.get_data_tlb().to_string_arquivo();
        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct SplitTLB.
    /// # Return
    /// * SplitTLB - Nova instancia da struct SplitTLB.
    pub fn new() -> SplitTLB
    {
        SplitTLB
        {
            instruction_tlb: InstructionTLB::new(),
            data_tlb: DataTLB::new(),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct InstructionTLB
{
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    memory_size: usize,
    replacement_algorithm: ReplacementAlgorithm,
}

impl InstructionTLB
{
    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Return
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

    /// Altera o valor do atributo cicles_per_access_write.
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

    /// Altera o valor do atributo replacement_algorithm.
    /// # Arguments
    /// * replacement_algorithm - Novo valor para o atributo replacement_algorithm.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm)
    {
        self.replacement_algorithm = replacement_algorithm;
    }

    /// Obtem o valor do atributo replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo replacement_algorithm.
    pub fn get_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.replacement_algorithm;
    }

    /// Converte uma enum do tipo InstructionTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo os atributos da enum InstructionTLB.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "Instruction TLB";
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
        dados += "\n";

        dados += "replacement algorithm:";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }

        return dados;
    }

    /// Converte uma struct do tipo InstructionTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        dados_para_arquivo += "\t\t<instructionTLB>\n";

        dados_para_arquivo += "\t\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "<memorySize>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t\t<replacementAlgorithm>";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</replacementAlgorithm>\n";

        dados_para_arquivo += "\t\t</instructionTLB>\n";

        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct InstructionTLB.
    /// # Return
    /// * InstructionTLB - Nova instancia da struct InstructionTLB.
    pub fn new() -> InstructionTLB
    {
        InstructionTLB
        {
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            memory_size: 0,
            replacement_algorithm: ReplacementAlgorithm::Null,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct DataTLB
{
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    memory_size: usize,
    replacement_algorithm: ReplacementAlgorithm,
}

impl DataTLB
{
    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Return
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
    /// * Novo valor para o atributo cicles_per_access_read.
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

    /// Altera o valor do atributo cicles_per_access_write.
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

    /// Altera o valor do atributo replacement_algorithm.
    /// # Arguments
    /// * replacement_algorithm - Novo valor para o atributo replacement_algorithm.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm)
    {
        self.replacement_algorithm = replacement_algorithm;
    }

    /// Obtem o valor do atributo replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo replacement_algorithm.
    pub fn get_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.replacement_algorithm;
    }

    /// Converte uma struct do tipo DataTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo os atributos da enum DataTLB.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "Data TLB";
        dados += "\n";

        dados += "memory size:";
        dados += &self.get_memory_size().to_string();
        dados += "\n";

        dados += "cicles per access read:";
        dados += &self.get_cicles_per_access_read().to_string();
        dados += "\n";

        dados += "cicles per access write:";
        dados += &self.get_cicles_per_access_write().to_string();
        dados += "\n";

        dados += "time cicle:";
        dados += &self.get_time_cicle().to_string();
        dados += "\n";

        dados += "replacement algorithm:";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }

        return dados;
    }

    /// Converte uma struct do tipo DataTLB em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        dados_para_arquivo += "\t\t<dataTLB>\n";

        dados_para_arquivo += "\t\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "<memorySize>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t\t<replacementAlgorithm>";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</replacementAlgorithm>\n";

        dados_para_arquivo += "\t\t</dataTLB>\n";

        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct DataTLB.
    /// # Return
    /// * DataTLB - Nova instancia da struct DataTLB.
    pub fn new() -> DataTLB
    {
        DataTLB
        {
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            memory_size: 0,
            replacement_algorithm: ReplacementAlgorithm::Null,
        }
    }
}