enum Status{
    Menu, //Significa que não foi identificado ainda a posição da caixa no mapa, então [0][0] é a posição nula definida
    ComCarteiro, // Siginifica que a posição da caixa é a mesma do carteiro, mas não será mostrada no mapa
    SemCarteiro // Siginifica que ela tem uma posição no mapa, mas NÃO é a mesma que a do carteiro
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