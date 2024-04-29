
use crate::components::carteiro::Carteiro;
use crate::components::caixa::Caixa;


pub struct Jogo{
    carteiro: Carteiro,
    caixa: Caixa
}


impl Jogo {
    pub fn new() -> Self {
        let carteiro = Carteiro::new();
        let caixa = Caixa::new();
        
        Jogo { carteiro, caixa } // tem que criar um retorno pra cada um "carteiro" e "caixa" que retorne
    }

    
    pub fn joga(&self){
        
        //Lógica escolhida para jogar

    }

    pub fn cria_jogo(&self){

    }
    }
