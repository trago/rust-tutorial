mod ml_data;

use crate::ml_data::read_ml_json;
use crate::ml_data::MLDataContainer;
use crate::ml_data::Node;
use std::path::Path;

fn test_list()
{
    // Pruebas de acceso a los archivos json
    let path = Path::new("C:/Users/jimfr/Documents/Maestria_Computacion/MaestriaComputacion/Primer_Semestre/Programacion_Algoritmos_I/rust-tutorial/resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    let path_match = Path::new("C:/Users/jimfr/Documents/Maestria_Computacion/MaestriaComputacion/Primer_Semestre/Programacion_Algoritmos_I/rust-tutorial/resources/1663154348643_8ZGUJJLLWV/current/1663154348643_8ZGUJJLLWV.json");
    let data_match = ml_data::read_ml_json(&path_match);

    // Imprime el numero de nodos en el segundo archivo
    let n = data_match.element_statistics.nodes.len();
    println!("{}", n );

    // Imprime la llave y el valor de la tabla hash del último nodo
    for (key,value) in &data_match.element_statistics.nodes[n-1].a{
        println!("{} {}", key, value);
    }

    // Itera sobre todos los nodos e imprime el nodo donde la tabla hash tenga el atributo 'XX:true'
    let iter = ml_data::read_ml_json(&path).element_statistics.nodes.into_iter().find(|node|{
        if let Some(XX) = node.a.get("XX"){
            if XX == "true" {return true}
        }
        return false
    });

    if let Some(datanode) = iter{
        print!("{:?}", datanode.a);
    }
}

// Funcion para encontrar un nodo con cierta etiqueta (filtrar)
pub fn filtrar_nodos ( tag:&String, data : &MLDataContainer ) -> Vec<Node>{
    // Creacion de un vector de nodos
    let mut nodes_vec : Vec<Node> = Vec::new();
    // Iteramos en los nodos
    for i in 0..data.element_statistics.nodes.len() {
        // y los que contengan la etiqueta tag
        if data.element_statistics.nodes[i].a.contains_key(tag){
            // me haces un clon de ese nodo a mi vector
            nodes_vec.push( data.element_statistics.nodes[i].clone() );
        }
    }
    return nodes_vec;
}

// Encontramos el nodo que tenga atributo XX
pub fn encontrar_XX( data : &MLDataContainer ) -> Option<Node> {
    // Iteramos sobre los nodos
    for i in 0..data.element_statistics.nodes.len() {
        // y aquel que tenga la llave XX en su hash map
        if data.element_statistics.nodes[i].a.contains_key("XX"){
            // lo tomamos
            return Some(data.element_statistics.nodes[i].clone());
        }
    }
    return None;
}

// Calculamos la correlación del nodo
pub fn correlacion_nodo( node_1:&Node, node_2:&Node ) -> f64 {
    let mut total:f64 = 0.0;
    let mut count:f64 = 0.0;
    let mut visited:Vec<String> = Vec::new();

    // recorremos las llaves de la tabla hash de cada nodo
    for key in node_1.a.keys(){
        if ( key != "WH" ) && ( key != "HT" ) && ( key != "XX" ) && ( key != "LT" ) & (key !=  "TP"){
            // las contamos
            total += 1.0;
            visited.push(key.to_string());
        }
        
        // Si coinciden las llaves de nuestros nodos de interes
        if node_2.a.contains_key( key ) && node_1.a[key] == node_2.a[key]{
            count += 1.0;
        }
    }
    
    // contamos los nodos 
    for key in node_2.a.keys(){
        if !visited.contains(key){
            total += 1.0;
        }
    }
    println!("{}/{}", count, total);
    return count/total;
}

// Creamos el vector donde vamos a guardar las correlaciones
pub fn correlation_vector( node_org:&Node, nodes_vecs:&Vec<Node> ) -> Vec<f64>{
    let mut i:usize=0;

    let mut correlation_vec:Vec<f64> = vec![0.0; nodes_vecs.len()];

    // Guardamos la correlacion
    for node in nodes_vecs{
        correlation_vec[i] = correlacion_nodo(&node_org, &node);
        i += 1;
    }

    return correlation_vec;
}

fn main(){
    // Lectura del archivo json
    let ruta = Path::new("C:/Users/jimfr/Documents/Maestria_Computacion/MaestriaComputacion/Primer_Semestre/Programacion_Algoritmos_I/rust-tutorial/resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    let DATA = read_ml_json(&ruta);

    // Lectura del archivo a comparar
    let path_curr = Path::new("C:/Users/jimfr/Documents/Maestria_Computacion/MaestriaComputacion/Primer_Semestre/Programacion_Algoritmos_I/rust-tutorial/resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    // desempaqueta un nodo empaquetado por Some()
    let xx_node = encontrar_XX( &DATA ).unwrap();

    let tag:String = "TV".to_string();
    let nodes_vec = filtrar_nodos( &tag, &DATA );
    let cor_vec = correlation_vector( &xx_node, &nodes_vec );

    let mut i:usize = 0;
    for j in 1..DATA.element_statistics.nodes.len(){
        if DATA.element_statistics.nodes[j].a.contains_key("XX"){
            i = j;
            break;
        }
    }

    //regresamos en pantalla el vector de correlacion
    for cor in cor_vec.iter(){
        println!("{}", cor);
    }

    let mut max = 0.0;
    for i in cor_vec.iter(){
        if( *i > max ){ max = *i };
    }

    let list_max:Vec<usize> = cor_vec.iter().enumerate().filter(|(_,corr)|{
        **corr == max
    }).map(|(index,_)|{
        index
    }).collect();
    
    for cor in list_max{
        println!("{}", cor);
    }  
}












