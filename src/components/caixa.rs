
pub struct Caixa{
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Caixa {
    pub fn new() -> Self{
        //aqui tem que retornar o objeto da Caixa

        Caixa {
            pos_x: 0,
            pos_y: 0,
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
}