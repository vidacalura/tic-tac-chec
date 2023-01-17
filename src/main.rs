use std::io;

struct DadosJogo {
    vez: String,
    lances: usize,
    tabuleiro: [[char; 4]; 4]
}

struct Jogador {
    peao: Peao,
    cavalo: Cavalo,
    torre: Torre,
    bispo: Bispo,
    pontos: u64,
    cor: String
}

struct Peao {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Peao {
    fn mover(){

    }
}

struct Cavalo {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Cavalo {
    fn mover(){

    }
}

struct Torre {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Torre {
    fn mover(){

    }
}

struct Bispo {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Bispo {
    fn mover(){

    }
}

struct Casa {
    x: Option<u8>,
    y: Option<u8>
}


fn main() {

    let dados_jogo = DadosJogo {
        vez: String::from("Brancas"),
        lances: 0,
        tabuleiro: [['.'; 4]; 4]
    };

    let player1 = jogador_constructor(String::from("Brancas")); 
    let player2 = jogador_constructor(String::from("Pretas"));

    // Primeira fase do jogo
    while (dados_jogo.lances <= 6) {
        let mut peca = String::new();
        println!("Insira qual peça deseja posicionar: \n(B (Bispo), N (Cavalo), R (Torre), P (Peão))");
        io::stdin()
            .read_line(&mut peca)
            .expect("Erro ao ler input.");

        let mut casa = String::new();
        println!("Insira a casa em que deseja posicionar esta peça:");
        io::stdin()
            .read_line(&mut casa)
            .expect("Erro ao ler input.");

        mostrar_tabuleiro(&dados_jogo.tabuleiro);

    }

    // Segunda fase do jogo
    loop {

    }

}


fn jogador_constructor(cor: String) -> Jogador {

    let notacao: [char; 4] = match &cor[..] {
        "Brancas" => ['P', 'N', 'R', 'B'],
        "Pretas" => ['p', 'n', 'r', 'b'],
        &_ => {
            println!("\nCor inválida\n");
            std::process::exit(1);
        }
    };

    Jogador {
        peao: Peao {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[0],
            cor: cor.clone()
        },
        cavalo: Cavalo {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[1],
            cor: cor.clone()
        },
        torre: Torre {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[2],
            cor: cor.clone()
        },
        bispo: Bispo {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[3],
            cor: cor.clone()
        },
        pontos: 0,
        cor
    }

}

fn mostrar_tabuleiro(tabuleiro: &[[char; 4]; 4]) {
    print!("\n");
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", tabuleiro[i][j]);
        }
        print!("\n");
    }
    print!("\n");
}
