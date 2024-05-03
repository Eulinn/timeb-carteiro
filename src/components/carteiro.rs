pub enum StatusCar {
    Menu, //Significa que não foi identificado ainda a posição do carteiro no mapa, então [0][0] é a posição nula definida
    JogandoSemCaixa, //Significa que a posição de interesse para o carteiro é a mesma da Caixa.
    JogandoComCaixa, //Significa que a posição de interesse para o carteiro é a do X
    Fim, //Significa que o carteiro chegou na mesma posição que o X e o jogo finaliza quando esse status for obtido.
}

pub struct Carteiro {
    pub pos_x: i32,
    pub pos_y: i32,
    pub status: StatusCar,
    pub caminho: Vec<(i32,i32)>
}

impl Carteiro {
    pub fn new() -> Self {
        //aqui tem que retorna o objeto do Carteiro

        Carteiro {
            pos_x: 0,
            pos_y: 0,
            status: StatusCar::Menu,
            caminho: vec![],
        }
    }

    //modifica a posição do carteiro
    pub fn set_posicao(&mut self, new_pos: Option<(usize, usize)>) -> bool {
        match new_pos {
            Some(posicoes) => {
                self.pos_x = posicoes.0 as i32;
                self.pos_y = posicoes.1 as i32;
                return true;
            },
            None => {
                return false;
            },
            
        }
    }

    //modifica o status do carteiro
    pub fn set_status(&mut self, new_status: StatusCar) {
        self.status = new_status
    }
}
