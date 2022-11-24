mod ml_data;
use std::path::Path;
use std::vec::Vec;
use crate::ml_data::MLDataContainer;
use crate::ml_data::Node;

// Funcion que encuentra los nodos que tienen la llave XX
pub fn encuentra_xx( data : &MLDataContainer ) -> Option<Node> {
    for i in 0..data.element_statistics.nodes.len() {
        //Si en nodo contiene la llave XX, entonces lo devuelvo
        if data.element_statistics.nodes[i].a.contains_key("XX"){
            return Some(data.element_statistics.nodes[i].clone());
        }
    }
    return None;
}

//Función que encuentra todos los nodos que contengan la llave TV
pub fn nodos_con_texto( tag:&String, data : &MLDataContainer ) -> Vec<Node>{
    let mut nodes_vec : Vec<Node> = Vec::new();
    for i in 0..data.element_statistics.nodes.len() {
        // Encontramos los nodos que contengan texto
        if data.element_statistics.nodes[i].a.contains_key(tag){
            nodes_vec.push( data.element_statistics.nodes[i].clone() );
        }
    }
    //Regresamos el nodo
    return nodes_vec;
}

// Funcion que obtiene la correlacion de un nodo con otro
pub fn correlacion_un_nodo(nodo_comparo:&Node, otro_nodo:&Node) -> f64 {
    //Cuenta el total de llaves distintas entre ambos nodos
    let mut total:f64 = 0.0;
    //Cuenta de los valores que tienen en comun
    let mut count:f64 = 0.0;
    // Vector donde junto las llaves de primer nodo para compararlas con las del segundo
    let mut llaves_nodo_comp:Vec<String> = Vec::new();

    for llave in nodo_comparo.a.keys(){
        if (llave != "WH") && (llave != "HT") && (llave != "TP") && (llave != "XX") && (llave != "LT") {
            //LLevamos la cuenta del total de llaves en el nodo
            total += 1.0;
            //Colecciono la llave
        llaves_nodo_comp.push(llave.to_string());
        }
        
        //Si el otro nodo tiene la misma llave y el mismo valor ent sumo 1
        if otro_nodo.a.contains_key( llave ) && nodo_comparo.a[llave]==otro_nodo.a[llave]{
            count += 1.0;
        }
    }

    // Vemos si el otro nodo tiene alguna llave que no este en el nodo a comparar
    for llave in otro_nodo.a.keys(){
        if !llaves_nodo_comp.contains(llave){
            //Sumo uno al total de llaves distintas
            total += 1.0;
        }
    }
    
    // Regresamos la proporcion de las cosas que tienen en común
    return count/total;

}

// Dunciones que calcula la correlacion entre todos los nodos
pub fn vector_correlacion( nodo_inicio:&Node , vector_nodos:&Vec<Node> ) -> Vec<f64>{
    let mut i:usize=0;
    // Creamos el vector donde vamos a guardar las correlaciones
    let mut vector_correlacion:Vec<f64> = vec![0.0; vector_nodos.len()];
    
    for nodo in vector_nodos{
        //guardo la correlacion
        vector_correlacion[i] = correlacion_un_nodo(&nodo_inicio, &nodo);
        i += 1;
    }
    // Regreso el vector
    return vector_correlacion;
}


fn main() {
    let mi_path = Path::new("/home/quaque/Cimat/Progralgoritmos/Proyectito/resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    
    let nodos_previo = ml_data::read_ml_json(mi_path);
    let nodos_actual = ml_data::read_ml_json(mi_path);
    let nodos_chidos = encuentra_xx(&nodos_previo).unwrap();

    //Para buscar el contexto busco en todo los cuadros que tengan texto, en TV viene el texto
    let tag:String = "TV".to_string();
    //Filtramos los nodos que contengan TV, en el archivo jason de prueba 
    let nodos_comparar = nodos_con_texto( &tag , &nodos_actual );
    //Sacamos el vector de correlacion
    let cor_vec = vector_correlacion( &nodos_chidos, &nodos_comparar );

    // Imprimo el vector
    for cor in cor_vec{
       println!("{}",cor);
    }
}
