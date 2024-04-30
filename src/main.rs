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
    pub mod jogo;
    mod carteiro;
    mod caixa;

}


use components::jogo::Jogo;


use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;


fn receber_mapa() -> Vec<Vec<char>> {


    //to obetendo o diretorio do projeto, e achando o mapa.txt
    let diretorio_atual = std::env::current_dir().expect("Não foi possível obtero do dir atual");
    let diretorio_atual = diretorio_atual.to_str().expect("Dificuldade em tranformar em string");

    let diretorio_completo:String = format!("{}\\src\\mapa.txt",diretorio_atual);    


    //Aqui eu peguei o arquivo e to lendo as linhas do buffer criado.
    let mapa_arquivo: File = File::open(diretorio_completo).expect("Há um erro na leitura do arquivo mapa.txt, erro");
    let interativo: BufReader<File> = BufReader::new(mapa_arquivo);


    let mut matriz:Vec<Vec<char>> = Vec::new();
    
    for linha in interativo.lines() {

        //To criando(ou não) um vetor de vetores que recebe o seguinte padrão: vetor[linha][coluna];
        match linha {
            Ok(valor) => {
                matriz.push(valor.chars().collect())
            },
            Err(e) =>{
                println!("Erro identificado: {}",e);
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
    
    //Atribuo o mapa aqui
   let mapa:Vec<Vec<char>> = receber_mapa();


    println!("{:?}",mapa);


    // let jogo: Jogo = Jogo::new();
    // jogo.cria_jogo(); // aqui vou botar o mapa como parâmetro
    // jogo.joga();

}
