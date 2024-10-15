/*
Utilize a biblioteca std::collections::HashMap para armazenar informações sobre os usuários, incluindo o nome, idade e lista de amigos.

Implemente uma interface de usuário simples usando o terminal para que os usuários possam escolher entre as seguintes opções:

Adicionar usuário

Adicionar amigo

Ver lista de amigos

Sair

Para adicionar um usuário, o programa deve pedir o nome e a idade do usuário e adicioná-los à HashMap.

Para adicionar um amigo, o programa deve pedir o nome do usuário e o nome do amigo e adicionar o amigo à lista de amigos do usuário.

Para visualizar a lista de amigos de um usuário, o programa deve pedir o nome do usuário e exibir a lista de amigos dele.

Quando o usuário escolher sair, o programa deve terminar.

Observação: As entradas do usuário devem ser lidas usando a função io::stdin().
*/



use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    friends: Vec<String>,
}

fn main() {
    let mut users = HashMap::new();

    loop {
        println!("Escolha uma opção:");
        println!("1. Adicionar usuário");
        println!("2. Adicionar amigo");
        println!("3. Ver lista de amigos");
        println!("4. Sair");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                println!("Digite o nome:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("Digite a idade:");
                let mut age = String::new();
                io::stdin().read_line(&mut age).unwrap();
                let age: u32 = age.trim().parse().unwrap();

                let user = User {
                    name: name.clone(),
                    age,
                    friends: Vec::new(),
                };

                users.insert(name, user);
                for (key, value) in &users {
                    println!("P4: iterando--> {}: {:?}",key,value);
                }
            
            },
            2 => {
                println!("Digite o nome do usuário:");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).unwrap();
                let user_name = user_name.trim().to_string();

                println!("Digite o nome do amigo:");
                let mut friend_name = String::new();
                io::stdin().read_line(&mut friend_name).unwrap();
                let friend_name = friend_name.trim().to_string();

                let user = users.get_mut(&user_name).unwrap();
                user.friends.push(friend_name);
                for (key, value) in &users {
                    println!("P4: iterando--> {}: {:?}",key,value);
                }
            },
            3 => {
                println!("Digite o nome do usuário:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                let user = users.get(&name).unwrap();
                println!("Amigos de {}:", name);
                for friend in &user.friends {
                    println!("- {}", friend);
                }
            },
            4 => {
                break;
                },
            _ => {
                println!("Opção inválida.");
                },
        }
    }
}
