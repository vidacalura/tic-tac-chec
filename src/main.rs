use std::io;

struct DadosJogo {
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
    fn mover(&mut self, x: usize, y: usize, tabuleiro: &mut [[char; 4]; 4]) -> Result<(), ()> {
        if x > 4 || x < 1 || y > 4 || y < 1 {
            return Err(());
        }

        match self.casa.y {
            // Move peão
            Some(_) => {
                 
            },
            // Coloca peão ainda não posicionado
            None => {
                match tabuleiro[y - 1][x - 1] {
                    '.' => {
                        self.casa.x = Some(x);
                        self.casa.y = Some(y);
                        tabuleiro[y - 1][x - 1] = self.notacao;
                    },
                    _ => {
                        println!("Casa já ocupada.");
                        return Err(());
                    }
                }
            }
        }

        return Ok(());
    }
}

struct Cavalo {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Cavalo {
    fn mover(&mut self, x: usize, y: usize, tabuleiro: &mut [[char; 4]; 4]) -> Result<(), ()> {
        if x > 4 || x < 1 || y > 4 || y < 1 {
            return Err(());
        }

        match self.casa.y {
            // Move cavalo
            Some(_) => {
                
            },
            // Coloca cavalo ainda não posicionado
            None => {
                match tabuleiro[y - 1][x - 1] {
                    '.' => {
                        self.casa.x = Some(x);
                        self.casa.y = Some(y);
                        tabuleiro[y - 1][x - 1] = self.notacao;
                    },
                    _ => {
                        println!("Casa já ocupada.");
                        return Err(());
                    }
                }
            }
        }

        return Ok(());
    }
}

struct Torre {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Torre {
    fn mover(&mut self, x: usize, y: usize, tabuleiro: &mut [[char; 4]; 4]) -> Result<(), ()> {
        if x > 4 || x < 1 || y > 4 || y < 1 {
            return Err(());
        }

        match self.casa.y {
            // Move torre
            Some(_) => {
                
            },
            // Coloca torre ainda não posicionada
            None => {
                match tabuleiro[y - 1][x - 1] {
                    '.' => {
                        self.casa.x = Some(x);
                        self.casa.y = Some(y);
                        tabuleiro[y - 1][x - 1] = self.notacao;
                    },
                    _ => {
                        println!("Casa já ocupada.");
                        return Err(());
                    }
                }
            }
        }

        return Ok(());
    }
}

struct Bispo {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Bispo {
    fn mover(&mut self, x: usize, y: usize, tabuleiro: &mut [[char; 4]; 4]) -> Result<(), ()> {
        if x > 4 || x < 1 || y > 4 || y < 1 {
            return Err(());
        }

        match self.casa.y {
            // Move bispo
            Some(_) => {
                
            },
            // Coloca bispo ainda não posicionado
            None => {
                match tabuleiro[y - 1][x - 1] {
                    '.' => {
                        self.casa.x = Some(x);
                        self.casa.y = Some(y);
                        tabuleiro[y - 1][x - 1] = self.notacao;
                    },
                    _ => {
                        println!("Casa já ocupada.");
                        return Err(());
                    }
                }
            }
        }

        return Ok(());
    }
}

struct Casa {
    x: Option<usize>,
    y: Option<usize>
}


fn main() {

    let mut dados_jogo = DadosJogo {
        lances: 0,
        tabuleiro: [['.'; 4]; 4]
    };

    let mut player1 = jogador_constructor(String::from("Brancas")); 
    let mut player2 = jogador_constructor(String::from("Pretas"));

    loop {
        let mut peca = String::new();
        println!("Insira qual peça deseja posicionar: \n(B (Bispo), N (Cavalo), R (Torre), P (Peão))");
        io::stdin()
            .read_line(&mut peca)
            .expect("Erro ao ler input.");

        let mut casa_y = String::new();
        println!("Insira a casa no eixo Y em que deseja posicionar esta peça:");
        io::stdin()
            .read_line(&mut casa_y)
            .expect("Erro ao ler input.");

        let casa_y: usize = match casa_y.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Posição do eixo Y inválida.");
                continue;
            }
        };

        let mut casa_x = String::new();
        println!("Insira a casa no eixo X em que deseja posicionar esta peça:");
        io::stdin()
            .read_line(&mut casa_x)
            .expect("Erro ao ler input.");

        let casa_x: usize = match casa_x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Posição do eixo Y inválida.");
                continue;
            }
        };

        let mut res: Result<(), ()> = Err(()); 
        match peca.trim().chars().nth(0).unwrap() {
            'p' | 'P' => {
                if dados_jogo.lances % 2 == 0 {
                    res = player1.peao.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
                else {
                    res = player2.peao.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
            },
            'n' | 'N' => {
                if dados_jogo.lances % 2 == 0 {
                    res = player1.cavalo.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
                else {
                    res = player2.cavalo.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
            },
            'r' | 'R' => {
                if dados_jogo.lances % 2 == 0 {
                    res = player1.torre.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
                else {
                    res = player2.torre.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
            },
            'b' | 'B' => {
                if dados_jogo.lances % 2 == 0 {
                    res = player1.bispo.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
                else {
                    res = player2.bispo.mover(casa_x, casa_y, &mut dados_jogo.tabuleiro);
                }
            },
            _ => {
                println!("Peça inválida");
                continue;
            }
        }

        match res {
            Ok(_) => print!(""),
            Err(_) => continue
        }

        dados_jogo.lances += 1;

        mostrar_tabuleiro(&dados_jogo.tabuleiro);

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
