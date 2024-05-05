use crate::components::caixa::Caixa;

use crate::components::carteiro::Carteiro;
use crate::components::carteiro::StatusCar;

use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
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

        Jogo { carteiro, caixa }
    }

    pub fn joga(&mut self, pos_x: i32, pos_y: i32, mapa: &Vec<Vec<char>>) {
        //Definir status do carteiro e da caixa
        self.carteiro.set_status(StatusCar::JogandoSemCaixa);

        //implemento o A*

        let mut ja_fui: Vec<(i32, i32)> = vec![(self.carteiro.pos_x, self.carteiro.pos_y)];
        let mut caminho_escolhido: Vec<(i32, i32)> = vec![(self.carteiro.pos_x, self.carteiro.pos_y)];
        let mut voltas = 0;
        let mut revoltas = 0;

        loop {
            let destino = match self.carteiro.status {
                StatusCar::JogandoSemCaixa => (self.caixa.pos_x, self.caixa.pos_y),
                StatusCar::JogandoComCaixa => (pos_x, pos_y),
                StatusCar::Fim => {
                    self.fim_de_jogo(mapa, caminho_escolhido);
                    break;
                }

                _ => (pos_x, pos_y),
            };

            let (vizinhos, interacao_obstaculo) =
                self.receber_vizinhos(mapa, self.carteiro.pos_x, self.carteiro.pos_y);
            let vizinhos = self.diferenca_de_vetores(ja_fui.clone(), vizinhos);

            let mut distancia_mais_proxima = f64::INFINITY;
            let mut vizinho_decente = None;

            voltas = if interacao_obstaculo {

                voltas + 1
            } else {

                0
            };

            

            println!("------------------");
            for vizinho in vizinhos.iter() {
                let distancia =
                    self.distancia_entre_pontos(vizinho.0, vizinho.1, destino.0, destino.1);
                if distancia < distancia_mais_proxima {
                    distancia_mais_proxima = distancia;
                    vizinho_decente = Some(*vizinho);
                }
                println!("Casa: {:?} distancia: {}", vizinho, distancia);
            }
            println!("Escolhido: {:?}", vizinho_decente);
            println!("------------------");


            ja_fui.extend(vizinhos.clone());

            self.carteiro
                .set_posicao(vizinho_decente.map(|(x, y)| (x as usize, y as usize)));

            if (self.carteiro.pos_y == 19
                || self.carteiro.pos_y == 0
                || self.carteiro.pos_x == 19
                || self.carteiro.pos_x == 0)
                && vizinhos.len() <= 1
                && interacao_obstaculo
            {
                if let Some((x_convert, y_convert)) =
                    caminho_escolhido.get(caminho_escolhido.len() - voltas)
                {
                    self.carteiro
                        .set_posicao(Some((*x_convert as usize, *y_convert as usize)));
                    let (mut vizinhos, _interacao_obstaculo) =
                        self.receber_vizinhos(mapa, self.carteiro.pos_x, self.carteiro.pos_y);

                    let mut distancia_mais_proxima = f64::INFINITY;
                    let mut vizinho_decente = None;

                    for vizinho in vizinhos.iter() {
                        let distancia =
                            self.distancia_entre_pontos(vizinho.0, vizinho.1, destino.0, destino.1);
                        if distancia < distancia_mais_proxima {
                            distancia_mais_proxima = distancia;
                            vizinho_decente = Some(*vizinho);
                        }
                    }

                    vizinhos.retain(|&x| x != vizinho_decente.unwrap());
                    vizinhos.retain(|&x| !caminho_escolhido.contains(&x));
                    ja_fui.retain(|&x| !vizinhos.contains(&x));
                    revoltas = 0;
                    voltas = 0;
                    continue;
                } else {
                    self.fim_de_jogo(mapa, caminho_escolhido);
                    println!("Não encontrei caminho válido!");
                    break;
                }
            }

            match caminho_escolhido.len() {
                0 => {}
                _ => {
                    if self.carteiro.pos_x != caminho_escolhido.last().unwrap().0 {
                        if mapa[caminho_escolhido.last().unwrap().0 as usize]
                            .iter()
                            .all(|&c| c == '+')
                        {
                            for coluna in 0..20 {
                                ja_fui.push((caminho_escolhido.last().unwrap().0, coluna));
                            }
                        }
                    }
                }
            }

            match vizinho_decente {
                Some(valor) => {
                    caminho_escolhido.push(valor);
                }
                None => {
                    self.fim_de_jogo(mapa, caminho_escolhido);
                    println!("Não encontrei caminho válido!");
                    break;
                }
            }

            if self.carteiro.pos_x == self.caixa.pos_x && self.carteiro.pos_y == self.caixa.pos_y {
                self.carteiro.set_status(StatusCar::JogandoComCaixa);
                ja_fui.clear();
            }

            if self.carteiro.pos_x == pos_x
                && self.carteiro.pos_y == pos_y
                && self.carteiro.status == StatusCar::JogandoComCaixa
            {
                self.carteiro.set_status(StatusCar::Fim);
            }

            // println!("Pos: {:?}", vizinho_decente);
            self.esperar_enter();
        }
    }

    fn diferenca_de_vetores(
        &self,
        lista_ja_fui: Vec<(i32, i32)>,
        vizinhos: Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let mut diferenca: Vec<(i32, i32)> = Vec::new();

        for elemento in vizinhos {
            if !lista_ja_fui.contains(&elemento) {
                diferenca.push(elemento);
            }
        }

        diferenca
    }

    fn distancia_entre_pontos(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        ((x2 - x1).pow(2) as f64 + (y2 - y1).pow(2) as f64).sqrt()
    }

    fn receber_vizinhos(
        &mut self,
        mapa: &Vec<Vec<char>>,
        pos_x: i32,
        pos_y: i32,
    ) -> (Vec<(i32, i32)>, bool) {
        let posicoes = vec![
            (pos_x - 1, pos_y), //cima
            (pos_x + 1, pos_y), //baixo
            (pos_x, pos_y - 1), //frente
            (pos_x, pos_y + 1), //costas
        ];

        let mut vizinhos_validos = Vec::new();

        let mut interacao_obstaculo = false;

        for posicao in posicoes {
            let (x, y) = posicao;
            if x >= 0 && y >= 0 && (x as usize) < mapa.len() && (y as usize) < mapa[0].len() {
                if mapa[x as usize][y as usize] != '-' {
                    vizinhos_validos.push(posicao);
                } else {
                    interacao_obstaculo = true;
                }
            }
        }

        let posicoes_diagonal = vec![
            (pos_x - 1, pos_y - 1), // diagonal superior esquerda
            (pos_x - 1, pos_y + 1), //diagonal superior direita
            (pos_x + 1, pos_y - 1), //diagonal inferior esquerda
            (pos_x + 1, pos_y + 1), //diagonal inferior direita
        ];

        for posicao in posicoes_diagonal {
            let (x, y) = posicao;
            if x >= 0 && y >= 0 && (x as usize) < mapa.len() && (y as usize) < mapa[0].len() {
                if mapa[x as usize][y as usize] == '-' {
                    interacao_obstaculo = true;
                }
            }
        }

        (vizinhos_validos, interacao_obstaculo)
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

        let (mut _pos_x, mut _pos_y): (i32, i32) = (0, 0);
        match posicao_final {
            Some(val) => {
                _pos_x = val.0 as i32;
                _pos_y = val.1 as i32;
            }
            None => {
                println!("Não foi possível identificar a posição final! \n Saindo...");
                process::exit(1);
            }
        }

        self.joga(_pos_x, _pos_y, mapa);

        // aqui os dois foram configurado
    }

    fn fim_de_jogo(&self, mapa: &Vec<Vec<char>>, caminho: Vec<(i32, i32)>) {
        let cor_caminho = Color::Red;
        let cor_mapa = Color::White;

        for (x, linha) in mapa.iter().enumerate() {
            for (y, coluna) in linha.iter().enumerate() {
                let cor = if caminho.contains(&(x as i32, y as i32)) {
                    cor_caminho
                } else {
                    cor_mapa
                };

                match execute!(std::io::stdout(), SetForegroundColor(cor)) {
                    Err(e) => {
                        println!("Erro ao pintar o mapa: {}", e);
                    }
                    _ => {}
                }

                print!("{}", coluna);

                match execute!(std::io::stdout(), SetForegroundColor(Color::Reset)) {
                    Err(e) => {
                        println!("Erro ao pintar o mapa: {}", e);
                    }
                    _ => {}
                }
            }

            println!();
        }
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
