enum Status{
    Menu,
    ComCarteiro,
    SemCarteiro
}

pub struct Caixa{
    pos_x: i32,
    pos_y: i32,
    status: Status
}

impl Caixa {
    pub fn new() -> Self{
        //aqui tem que retornar o objeto da Caixa

        Caixa {
            pos_x: 0,
            pos_y: 0,
            status: Status::Menu
        }
    }
}