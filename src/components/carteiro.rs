enum Status{
    Menu, //Significa que não foi identificado ainda a posição do carteiro no mapa, então [0][0] é a posição nula definida
    JogandoSemCaixa, //Significa que a posição de interesse para o carteiro é a mesma da Caixa.
    JogandoComCaixa, //Significa que a posição de interesse para o carteiro é a do X
    Fim //Significa que o carteiro chegou na mesma posição que o X e o jogo finaliza quando esse status for obtido.
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