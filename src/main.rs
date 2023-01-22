use std::io;

struct DadosJogo {
    lances: usize,
    tabuleiro: [[Option<char>; 4]; 4]
}

struct Peca {
    casa: Casa,
    notacao: char,
    cor: String
}

impl Peca {
    fn mover(&mut self, x: usize, y: usize, tabuleiro: &mut [[Option<char>; 4]; 4]) -> Result<(), ()> {
        if x > 3 || y > 3 {
            return Err(());
        }

        match self.casa.y {
            // Move peça
            Some(_) => {
                match self.notacao {
                    'p' => {
                        // Mover peão para frente
                        if y == self.casa.y.unwrap() + 1
                        && x == self.casa.x.unwrap() {
                            tabuleiro[self.casa.y.unwrap()][self.casa.x.unwrap()] = None;
                            tabuleiro[y][x] = Some(self.notacao);
                        }
                        else if y == self.casa.y.unwrap() + 1
                        && x == self.casa.x.unwrap() + 1 {
                            // Analisa se há uma peça nesta casa para captura
                            match tabuleiro[y - 1][x - 1] {
                                Some(v) => {
                                    // Verifica se peça é do adversário
                                        tabuleiro[y - 1][x - 1] = None;
                                        tabuleiro[y][x] = Some(self.notacao);
                                        // Remover casa.x e casa.y da peça que estava na casa
                                },
                                None => {
                                    println!("\nMovimento inválido\n");
                                    return Err(());
                                }
                            }
                        }
                        else if y == self.casa.y.unwrap() + 1
                        && x + 1 == self.casa.x.unwrap() {
                            // Analisa se há uma peça nesta casa para captura
                            match tabuleiro[y - 1][x + 1] {
                                Some(_) => {
                                    tabuleiro[y - 1][x + 1] = None;
                                    tabuleiro[y][x] = Some(self.notacao);
                                    // Remover casa.x e casa.y da peça que estava na casa
                                },
                                None => {
                                    println!("\nMovimento inválido\n");
                                    return Err(());
                                }
                            }
                        }
                        else {
                            println!("\nMovimento inválido!\n");
                            return Err(());
                        }
                    },
                    'P' => {
                        if y + 1 == self.casa.y.unwrap()
                        && x == self.casa.x.unwrap() {
                            tabuleiro[self.casa.y.unwrap()][self.casa.x.unwrap()] = None;
                            tabuleiro[y][x] = Some(self.notacao);
                        }
                        else if y + 1 == self.casa.y.unwrap()
                        && x == self.casa.x.unwrap() + 1 {
                            // Analisa se há uma peça nesta casa para captura
                            match tabuleiro[y + 1][x - 1] {
                                Some(_) => {
                                    tabuleiro[y + 1][x - 1] = None;
                                    tabuleiro[y][x] = Some(self.notacao);
                                    // Remover casa.x e casa.y da peça que estava na casa
                                },
                                None => {
                                    println!("\nMovimento inválido\n");
                                    return Err(());
                                }
                            }
                        }
                        else if y + 1 == self.casa.y.unwrap()
                        && x + 1 == self.casa.x.unwrap() {
                            // Analisa se há uma peça nesta casa para captura
                            match tabuleiro[y + 1][x + 1] {
                                Some(_) => {
                                    tabuleiro[y + 1][x + 1] = None;
                                    tabuleiro[y][x] = Some(self.notacao);
                                    // Remover casa.x e casa.y da peça que estava na casa
                                },
                                None => {
                                    println!("\nMovimento inválido\n");
                                    return Err(());
                                }
                            }
                        }
                        else {
                            println!("\nMovimento inválido!\n");
                            return Err(());
                        }

                    },
                    'n' | 'N' => {
                        
                    },
                    'r' | 'R' => {

                    },
                    'b' | 'B' => {

                    },
                    _ => {
                        println!("Erro durante o processamento do jogo");
                        std::process::exit(1);
                    }
                }
            },
            // Coloca peça ainda não posicionada
            None => {
                match tabuleiro[y][x] {
                    None => {
                        self.casa.x = Some(x);
                        self.casa.y = Some(y);
                        tabuleiro[y][x] = Some(self.notacao);
                    },
                    Some(_) => {
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

struct Jogador {
    peao: Peca,
    cavalo: Peca,
    torre: Peca,
    bispo: Peca,
    pontos: u64,
    cor: String
}

fn main() {

    let mut dados_jogo = DadosJogo {
        lances: 0,
        tabuleiro: [[None; 4]; 4]
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
 
        let is_vez_brancas: bool = dados_jogo.lances % 2 == 0;
        let player_pointer = if is_vez_brancas { &mut player1 } else { &mut player2 };

        let mut res: Result<(), ()> = Err(());
        match peca.trim().chars().nth(0).unwrap() {
            'p' | 'P' => {
                // -1 se deve pois quando os jogadores escolhem uma casa, escolhem
                // contando a partir do index 1, a máquina, porém, começa do index 0
                res = player_pointer.peao.mover(casa_x - 1, casa_y - 1, &mut dados_jogo.tabuleiro);
            },
            'n' | 'N' => {
                res = player_pointer.cavalo.mover(casa_x - 1, casa_y - 1, &mut dados_jogo.tabuleiro);
            },
            'r' | 'R' => {
                res = player_pointer.torre.mover(casa_x - 1, casa_y - 1, &mut dados_jogo.tabuleiro);
            },
            'b' | 'B' => {
                res = player_pointer.bispo.mover(casa_x - 1, casa_y - 1, &mut dados_jogo.tabuleiro);
            },
            _ => {
                println!("Peça inválida");
                continue;
            }
        }

        match res {
            Ok(_) => (),
            Err(_) => {
                println!("");
                continue;
            }
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
        peao: Peca {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[0],
            cor: cor.clone()
        },
        cavalo: Peca {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[1],
            cor: cor.clone()
        },
        torre: Peca {
            casa: Casa {
                x: None,
                y: None
            },
            notacao: notacao[2],
            cor: cor.clone()
        },
        bispo: Peca {
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

fn mostrar_tabuleiro(tabuleiro: &[[Option<char>; 4]; 4]) {
    print!("\n");
    for i in 0..4 {
        print!("{} ", i + 1);
        for j in 0..4 {
            match tabuleiro[i][j] {
                None => print!(". "),
                Some(p) => print!("{} ", p)
            }
        }
        print!("\n");
    }
    println!("  1 2 3 4");
    print!("\n");
}
