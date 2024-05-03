pub enum StatusCai{
    Menu, //Significa que não foi identificado ainda a posição da caixa no mapa, então [0][0] é a posição nula definida
    ComCarteiro, // Siginifica que a posição da caixa é a mesma do carteiro, mas não será mostrada no mapa
    SemCarteiro // Siginifica que ela tem uma posição no mapa, mas NÃO é a mesma que a do carteiro
}

pub struct Caixa{
    pub pos_x: i32,
    pub pos_y: i32,
    pub status: StatusCai
}

impl Caixa {
    pub fn new() -> Self{
        //aqui tem que retornar o objeto da Caixa

        Caixa {
            pos_x: 0,
            pos_y: 0,
            status: StatusCai::Menu
        }
    }

    //modifica a posição da caixa
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

    //modifica o status da caixa
    pub fn set_status(&mut self, new_status: StatusCai){
        self.status = new_status;
    }
}