enum Status{
    Menu,
    JogandoSemCaixa,
    JogandoComCaixa,
    Fim
}

pub struct Carteiro{
    pos_x: i32,
    pos_y: i32,
    status: Status
}

impl Carteiro{
    pub fn new() -> Self {
    //aqui tem que retorna o objeto do Carteiro


    Carteiro {
        pos_x: 1,
        pos_y: 1,
        status: Status::Menu
    }
    }
}