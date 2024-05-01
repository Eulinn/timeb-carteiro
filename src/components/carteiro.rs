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
        pos_x: 0,
        pos_y: 0,
        status: Status::Menu
    }
    }

}