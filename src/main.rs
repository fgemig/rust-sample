use chrono::{DateTime, Local};
use rand::Rng;

fn main() {

    let _data_atual: DateTime<Local> = Local::now();
    let _dia_atual = _data_atual.format("%d").to_string();

    let dia = _dia_atual.parse::<i32>().unwrap();
    
    let retorno = dia_de_comer_bolo(dia);

    exibir_mensagem(retorno);
}

fn dia_de_comer_bolo(dia: i32) -> bool {

    let _dias_do_mes:[i32;6] = [1, 3, 6, 12, 18, 24];    
       
    let _dia_de_comer_bolo = _dias_do_mes.iter().any(|&i| i== dia);

    return _dia_de_comer_bolo;
}

fn exibir_mensagem(comer: bool) {

    if comer {

        let sabor = sabor_de_hoje();

        println!("Hoje é dia de comer bole de: {0} ", sabor);

        return;
    }

    println!("Hoje não é dia de comer bolo :(");
}

fn sabor_de_hoje() -> String {

    let sabores = vec!["chocolate", "cenoura", "fubá", "baunilha", "laranja"];

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0, sabores.len());

    let sabor = sabores[random];

    return sabor.to_string();
}