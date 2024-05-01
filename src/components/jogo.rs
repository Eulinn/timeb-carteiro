use crate::components::caixa::Caixa;
use crate::components::carteiro::Carteiro;


use std::process;
use crossterm::{terminal::{Clear, ClearType}, ExecutableCommand};
use std::io::{self, Write};

pub struct Jogo {
    carteiro: Carteiro,
    caixa: Caixa,
}

impl Jogo {
    pub fn new() -> Self {
        let carteiro = Carteiro::new();
        let caixa = Caixa::new();

        Jogo { carteiro, caixa } // tem que criar um retorno pra cada um "carteiro" e "caixa" que retorne
    }

    pub fn joga(&self) {

        //Lógica escolhida para jogar
    }

    fn cria_jogo(&self, mapa: &Vec<Vec<char>>) {

        // aqui vou pegar a posição do carteiro, da caixa e do destino (todos obrigatórios) e atualizar nas suas classes
    }



    pub fn menu_jogo(&self, mapa: &Vec<Vec<char>>) {
        loop {
            let mut opt: String = String::new();
            println!("Bem vindos ao jogo de missão de entrega. Aqui você verá o caminho");
            println!("percorrido pelo entregador ao buscar uma caixa e levar ao local");
            println!("desejado.");
            println!("Selecione uma opção: ");
            println!("(A) - Jogar\n(B) - Regras\n(C) - Sair");

            //Faz o pedido de uma opção pro usuário e trata essa opção da forma correta
            match io::stdin().read_line(&mut opt) {
                Ok(_) => match opt.trim().to_lowercase().as_str() {
                    "a" => {
                        self.limpar_tela();
                        
                        break; //Único break do loop, ou o jogo roda ou o arquivo fecha, sem opções
                    }
                    "b" => self.mostrar_regras(),
                    "c" => {
                        println!("Saindo...");
                        process::exit(0);
                    }
                    _ => {
                        self.limpar_tela();
                    }
                },
                Err(e) => {
                    println!("Erro ao identificar seu valor digitado: {}", e);
                    process::exit(1);
                }
            }
        }
    }

    //Facilitar a visualização do conteúdo sem poluição.
    fn limpar_tela(&self) {
        io::stdout().execute(Clear(ClearType::All)).unwrap();
        io::stdout().flush().unwrap();
    }

    //caso de algum erro no enter, é culpa desta função.
    fn esperar_enter(&self) {
        let _ = io::stdin().read_line(&mut String::new());
    }

    //essa função pode ser chamada quando quiser para saber as regras do jogo
    fn mostrar_regras(&self) {
        self.limpar_tela();
        println!("O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até ó ponto desejado ('X')
em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido..
Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.
Obs:
    - O carteiro só pode andar um 'índice' por iteração
    - Apliquem a ideia de Encapsulamento
    - O código tera uma mapa de exemplo para o teste enquanto estiver em desenvolvimento
    - No dia da apresentação o código será posto em prática com um código diferente");
        println!("Aperte ENTER para voltar ao menu!");
        self.esperar_enter();
        self.limpar_tela();
    }





}
