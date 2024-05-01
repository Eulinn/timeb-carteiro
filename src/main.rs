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
use crossterm::ExecutableCommand;

use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};
use std::process;
use crossterm::terminal::{Clear, ClearType};

fn receber_mapa() -> Vec<Vec<char>> {
    //to obetendo o diretorio do projeto, e achando o mapa.txt
    let diretorio_atual =
        std::env::current_dir().expect("Não foi possível obter o diretório atual.");
    let diretorio_atual = diretorio_atual
        .to_str()
        .expect("Dificuldade em tranformar em string seu diretório.");

    let diretorio_completo: String = format!("{}\\src\\mapa.txt", diretorio_atual);


    //Aqui eu peguei o arquivo e to lendo as linhas do buffer criado.
    let mapa_arquivo: File =
        File::open(diretorio_completo).expect("Há um erro na leitura do arquivo mapa.txt, erro");
    let interativo: BufReader<File> = BufReader::new(mapa_arquivo);

    let mut matriz: Vec<Vec<char>> = Vec::new();

    for linha in interativo.lines() {
        //To criando(ou não) um vetor de vetores que recebe o seguinte padrão: vetor[linha][coluna];
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

fn limpar_tela() {

    io::stdout().execute(Clear(ClearType::All)).unwrap();
    io::stdout().flush().unwrap();

}

fn esperar_enter() {
    let _ = io::stdin().read_line(&mut String::new());
}


fn mostra_mapa_bonito(mapa: Vec<Vec<char>>){

    for linhas in mapa {
        for coluna in linhas.iter(){
            print!("{}   ",coluna);
        };
        println!("\n");   
    }
}




fn main() {
    loop {
        let mut opt = String::new();
        println!("Jogo de missão de entrega. Selecione a opção desejada e aperte Enter: ");
        println!("(A) - Jogar\n(B) - Regras\n(C) - Sair");


        match io::stdin().read_line(&mut opt) {
            Ok(_) => match opt.trim().to_lowercase().as_str() {
                "a" => {
                    limpar_tela();
                    println!("Vamos, antes, verificar se seu mapa está compatível com nosso jogo!

Aperte ENTER para fazer a verificação!");
                    
                    esperar_enter();
                    let mapa: Vec<Vec<char>> = receber_mapa();
                    
                    limpar_tela();
                    println!("Este é o mapa que seu carteiro(&) deve percorrer:");
                    mostra_mapa_bonito(mapa);
                    println!("Aperte ENTER para iniciar o jogo");
                    esperar_enter();
                    break;
                }
                "b" => {
                    limpar_tela();

                    println!("O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até ó ponto desejado ('X') em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido..
Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.
Obs:
    - O carteiro só pode andar um 'índice' por iteração
    - Apliquem a ideia de Encapsulamento
    - O código tera uma mapa de exemplo para o teste enquanto estiver em desenvolvimento
    - No dia da apresentação o código será posto em prática com um código diferente");
                    println!("Aperte ENTER para voltar ao menu!");
                    esperar_enter();
                    limpar_tela();
                }
                "c" => {
                    println!("Saindo...");
                    process::exit(0);
                }
                _ => {
                    limpar_tela();
                }
            },
            Err(e) => {
                println!("Erro ao identificar seu valor digitado: {}", e);
                process::exit(1);
            }
        }
    }
    limpar_tela();
    println!("Jogo Começa aqui"); // Jogo começa aqui


    // let jogo: Jogo = Jogo::new();
    // jogo.cria_jogo(); // aqui vou botar o mapa como parâmetro
    // jogo.joga();
}
