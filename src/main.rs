#![feature(proc_macro_hygiene, decl_macro)]
extern crate postgres;
//use config::Result;
use csv;
use postgres::{Client, NoTls};
use rocket::*;
use std::error::Error;
//use serde_derive::{Deserialize};

//#[derive(Debug, Deserialize)]
struct Persona {
    identificacion: String,
    nombre: String,
    genero: String,
    estado_civil: String,
    fecha_nacimiento: String,
    telefono: String,
    direccion: String,
    email: String,
}

//leer un archivo csv
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let me = Persona {
        identificacion: "1".to_owned(),
        nombre: "prueba".to_owned(),
        genero: "Masculino".to_owned(),
        estado_civil: "Soltero".to_owned(),
        fecha_nacimiento: "1995/11/13".to_owned(),
        telefono: "1234567890".to_owned(),
        direccion: "latacunga cot".to_owned(),
        email: "prueba@gmail.com".to_owned(),
    };
    let mut client = Client::connect(
        "postgresql://postgres:Titos95@localhost:5432/registro",
        NoTls,
    )
    .expect("Connection failed");
    for row in &client.query("SELECT * FROM persona", &[]).unwrap() {
        println!("Entro");
        println!("{:?}", row);
    }
    client
        .execute(
            "INSERT INTO persona (identificacion, nombre,genero,estado_civil,fecha_nacimiento,telefono,direccion,email) VALUES ($1, $2,$3,$4,$5,$6,$7,$8)",
            &[
                &me.identificacion,
                &me.nombre,
                &me.genero,
                &me.estado_civil,
                &me.fecha_nacimiento,
                &me.telefono,
                &me.direccion,
                &me.email,
            ],
        )
        .unwrap();
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);
    for result in reader.records() {
        //for result in reader.deserialize(){
        let record = result?;
        //let record: Persona = result?;

        println!("{:?}", record.get(0));
    }

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    if let Err(e) = read_from_file("D:/Programaciòn Rust/REGISTRO.csv") {
        eprintln!("{}", e);
    }
    //rocket::ignite().mount("/", routes![index]).launch();
}
