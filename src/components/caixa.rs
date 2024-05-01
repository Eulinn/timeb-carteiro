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

    //modifica a posição da caixa
    pub fn set_posicao(caixa: &mut Caixa, new_x: i32, new_y: i32){
        caixa.pos_x = new_x;
        caixa.pos_y = new_y;
    }

    //modifica o status da caixa
    pub fn set_status(caixa: &mut Caixa, new_status: Status){
        caixa.status = new_status;
    }
}