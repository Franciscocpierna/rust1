use std::io;

fn convert_to_int(data_input: & String)->i32{
    let x=data_input.trim().parse::<i32>().unwrap(); 
    x 
}

   fn main() {
     let mut medias=String::new(); 
     println!("ente com numero medias : ");
     io::stdin().read_line(&mut medias).expect("Erro ao ler medias");
     let mut soma_rec = 0;
     let mut i = 0;
     while convert_to_int(&medias) > i{
        let mut media_aluno= String::new();
        println!("ente com as medias dos alunos: ");
        io::stdin().read_line(&mut media_aluno).expect("erro ao lert media_aluno");
        i+=1;
        if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno)< 6{
            soma_rec +=1;
        }
        
     }
     println!("o numero de alunos em recuperação é {}", soma_rec);

   
   }