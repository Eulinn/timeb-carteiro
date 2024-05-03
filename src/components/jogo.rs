use crate::components::caixa::Caixa;
use crate::components::carteiro::Carteiro;

use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};
use std::process;

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

    pub fn joga(&mut self, pos_x: i32, pos_y: i32, mapa: &Vec<Vec<char>>) {
        //Definir status do carteiro e da caixa



        
        //Aplicar a nossa versão da lógica do A* aqui!

        loop {
            let vizinhos = self.receber_vizinhos(mapa);
            
            //if para verificar o status do carteiro se vai pro final ou pra caixa
            for vizinho in vizinhos.iter() {

                    //if para verificar qual é o mais perto dos vizinhos e pular para o tal

            }


        }




    }



    fn receber_vizinhos(&mut self, mapa: &Vec<Vec<char>>) -> Vec<(i32,i32)>{
        let mut posicoes = vec![
            (self.carteiro.pos_x-1, self.carteiro.pos_y),//cima
            (self.carteiro.pos_x+1, self.carteiro.pos_y),//baixo
            (self.carteiro.pos_x, self.carteiro.pos_y-1),//frente
            (self.carteiro.pos_x, self.carteiro.pos_y+1),//costas
        ];


        let mut index = 0;
        while index < posicoes.len() {
            if posicoes[index].0 < 0 || posicoes[index].1 < 0 {
                posicoes.remove(index);
                continue;
            }

            if mapa[posicoes[index].0 as usize][posicoes[index].1 as usize] == '-' {
                posicoes.remove(index);
                continue;
            }


            index+=1;            
        }
        


        

        return posicoes;
    }



    fn cria_jogo(&mut self, mapa: &Vec<Vec<char>>) {
        let (posicao_carteiro, posicao_caixa, posicao_final) = self.retorna_posicao(mapa);

        if !self.carteiro.set_posicao(posicao_carteiro) {
            println!("Não foi possível identificar a posição do carteiro!\n Saindo...");
            process::exit(1);
        }

        if !self.caixa.set_posicao(posicao_caixa) {
            println!("Não foi possível identificar a posição da caixa! \n Saindo...");
            process::exit(1);
        }

        let (mut pos_x, mut pos_y): (i32, i32) = (0, 0);
        match posicao_final {
            Some(val) => {
                pos_x = val.0 as i32;
                pos_y = val.1 as i32;
            }
            None => {
                println!("Não foi possível identificar a posição final! \n Saindo...");
                process::exit(1);
            }
        }

        self.joga(pos_x, pos_y, mapa);

        // aqui os dois foram configurado
    }

    fn retorna_posicao(
        &self,
        matriz: &Vec<Vec<char>>,
    ) -> (
        Option<(usize, usize)>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
    ) {
        let (mut carteiro, mut caixa, mut finall) = (None, None, None);

        for (linha, linha_chars) in matriz.iter().enumerate() {
            for (coluna, &caractere) in linha_chars.iter().enumerate() {
                match caractere {
                    '&' => carteiro = Some((linha, coluna)),
                    '@' => caixa = Some((linha, coluna)),
                    'X' => finall = Some((linha, coluna)),
                    _ => {}
                }
            }
        }

        (carteiro, caixa, finall)
    }

    pub fn menu_jogo(&mut self, mapa: &Vec<Vec<char>>) {
        loop {
            println!("Bem vindos ao jogo de missão de entrega. Aqui você verá o caminho");
            println!("percorrido pelo entregador ao buscar uma caixa e levar ao local");
            println!("desejado.");
            println!("Selecione uma opção: ");
            println!("(A) - Jogar\n(B) - Regras\n(C) - Sair");

            let opt = match self.ler_usuario() {
                Ok(opt) => opt,
                Err(e) => {
                    self.limpar_tela();
                    println!("Erro ao ler a opção: {}", e);
                    continue;
                }
            };

            match opt.trim().to_lowercase().as_str() {
                "a" => {
                    self.limpar_tela();
                    self.cria_jogo(mapa);
                    break;
                }
                "b" => self.mostrar_regras(),
                "c" => {
                    println!("Saindo...");
                    process::exit(0);
                }
                _ => self.limpar_tela(),
            }
        }
    }

    fn ler_usuario(&self) -> io::Result<String> {
        print!("Opção: ");
        io::stdout().flush()?;
        let mut opt = String::new();
        io::stdin().read_line(&mut opt)?;
        Ok(opt)
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
        const REGRAS: &str = r#"
O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até o ponto desejado ('X')
em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido.

Para desenvolver tal projeto, vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.

Regras:
- O carteiro só pode andar um 'índice' por iteração.
- Apliquem a ideia de Encapsulamento.
- O código terá um mapa de exemplo para o teste enquanto estiver em desenvolvimento.
- No dia da apresentação, o código será posto em prática com um código diferente.
"#;

        println!("{}", REGRAS);
        println!("Aperte ENTER para voltar ao menu!");
        self.esperar_enter();
        self.limpar_tela();
    }
}
