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
use crate::fronteira::{bem_vindo, obter_comando};
use crate::fronteira::alterar_arquitetura::alterar_arquitetura;
use crate::fronteira::criar_arquitetura::criar_arquitetura;
use crate::fronteira::exibir_arquitetura::exibir_arquitetura;
use crate::fronteira::salvar_arquivo::salvar_arquitetura_arquivo;

mod arquitetura;
mod fronteira;

/// Executa o programa.
fn main()
{
    bem_vindo();

    //Criando uma variavel para armazenar as configuracoes do Amnesia
    let mut arquitetura = Arquitetura::new();

    //Obtendo e executando o comando do usuario
    loop
    {
        let comando = obter_comando();

        if comando == 0
        {
            break;
        } else if comando == 1
        {
            criar_arquitetura(&mut arquitetura);
        } else if comando == 2
        {
            exibir_arquitetura(&arquitetura);
        } else if comando == 3
        {
            alterar_arquitetura(&mut arquitetura)
        } else if comando == 4
        {
            salvar_arquitetura_arquivo(&arquitetura);
        }
    }
}