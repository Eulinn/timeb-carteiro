//  JOGO DA ENTREGA
//
// O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até ó ponto desejado ('X') em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido..
// Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.
// Obs:
//   - O carteiro só pode andar um 'índice' por iteração
//   - Apliquem a ideia de Encapsulamento
//   - O código tera uma mapa de exemplo para o teste enquanto estiver em desenvolvimento
//   - No dia da apresentação o código será posto em prática com um código diferente

mod components {
    mod caixa;
    mod carteiro;
    pub mod jogo;
}

use components::jogo::Jogo;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn receber_mapa() -> Vec<Vec<char>> {
    //to obetendo o diretorio do projeto, e achando o mapa.txt
    let diretorio_completo = std::env::current_dir()
        .expect("Não foi possível obter o diretório atual.")
        .join("src")
        .join("mapa.txt");

    //Aqui eu peguei o arquivo e to lendo as linhas do buffer criado.
    let mapa_arquivo: File =
        File::open(&diretorio_completo).expect("Há um erro na leitura do arquivo mapa.txt, erro");


    let interativo: BufReader<File> = BufReader::new(mapa_arquivo);
    let mut matriz: Vec<Vec<char>> = Vec::new();



    //To criando(ou não) um vetor de vetores que recebe o seguinte padrão: vetor[linha][coluna];
    for linha in interativo.lines() {
        match linha {
            Ok(valor) => matriz.push(valor.chars().collect()),
            Err(e) => {
                println!("Erro identificado: {}", e);
                process::exit(1);
            }
        }
    }
    /*
       OBS:
       * Atualizar o mapa.txt muda o mapa do jogo, é só rodar de novo que já atualiza no código;
       * O arquivo deve estar na pasta src junto com nosso main.rs
    */
    return matriz;
}



fn main() {

    let mut jogo: Jogo = Jogo::new(); // Cria a instancia do jogo
    jogo.menu_jogo(&receber_mapa()); //Cria a lógica do jogo já passando o mapa como parâmetro

}
