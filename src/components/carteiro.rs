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

    //modifica a posição do carteiro
    pub fn set_posicao(&mut self, new_x: i32, new_y: i32){
        self.pos_x = new_x;
        self.pos_y = new_y;
    }
    
    //modifica o status do carteiro
    pub fn set_status(&mut self, new_status: Status){
        self.status = new_status
    }
}