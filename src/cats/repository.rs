// #![allow(proc_macro_derive_resolution_fallback)]
// use crate::cats::{Cat, InsertableCat};
// use crate::mongo_connection::Conn;
// // use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
// // use mongodb::{bson, results::DeleteResult, bson::doc, error::ErrorKind, bson::oid::ObjectId};
// // use futures::stream::StreamExt;
// use mongodb::bson;
// use mongodb::{
//     bson::{doc, Bson},
//     options::FindOptions,
//     bson::oid::ObjectId,
//     error::Error,
// };

// const COLLECTION: &str = "cats";
// // -> Result<Vec<Cat>, ErrorKind>
// pub fn all(connection: &Conn) -> Result<Vec<Cat>, r2d2_mongodb::mongodb::Error> {
//     // let mut cursor = connection.collection(COLLECTION).find(None, None).unwrap();
//     // let docs = vec![
//     // doc! { "title": "1984", "author": "George Orwell" },
//     // doc! { "title": "Animal Farm", "author": "George Orwell" },
//     // doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//     // ];

//     // Insert some documents into the "mydb.books" collection.
//     // connection.collection(COLLECTION).insert_many(docs, None)?;

//     let mut cursor = connection.collection(COLLECTION).find(None, None)?;
//     println!("HELLO");
//     for result in cursor {
//         match result {
//             Ok(document) => {
//                 if let Some(title) = document.get("title").and_then(r2d2_mongodb::mongodb::Bson::as_str) {
//                     println!("title: {}", title);
//                 } else {
//                     println!("no title found");
//                 }
//             }
//             Err(e) => return Err(e.into()),
//         }
//     }
//     println!("POOP");
//     let a: Vec<Cat> = Vec::new();
//     Ok(a)
//     // while let Ok(result) = cursor.next().unwrap() {
//     //     match result {
//     //         Ok(document) => {
//     //             if let Some(title) = document.get("title").and_then(Bson::as_str) {
//     //                 println!("title: {}", title);
//     //             }  else {
//     //                 println!("no title found");
//     //             }
//     //         }
//     //         // Err(e) => return Err(e.into()),
//     //         Err(_) => return (),
//     //     }
//     // }
//     // cursor
//     //     .map(|result| match result {
//     //         Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
//     //             Ok(result_model) => Ok(result_model),
//     //             Err(_) => Err(ErrorKind::InternalError{message: String::from("")}),
//     //         },
//     //         Err(err) => Err(err),
//     //     })
//     //     .collect::<Result<Vec<Cat>, ErrorKind>>()
// }

// // pub fn get(id: ObjectId, connection: &Conn) -> Result<Option<Cat>, ErrorKind> {
// //     match connection
// //         .collection(COLLECTION)
// //         .find_one(Some(doc! {"_id": id}), None)
// //     {
// //         Ok(db_result) => match db_result {
// //             Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
// //                 Ok(result_model) => Ok(Some(result_model)),
// //                 Err(_) => Err(ErrorKind::InternalError{message: String::from(
// //                     "Failed to create reverse BSON",
// //                 )}),
// //             },
// //             None => Ok(None),
// //         },
// //         Err(err) => Err(err),
// //     }
// // }

// // pub fn insert(cats: Cat, connection: &Conn) -> Result<ObjectId, mongodb::bson::de::Error> {
// //     let insertable = InsertableCat::from_cat(cats.clone());
// //     match r2d2_mongodb::mongodb::Bson::to_bson(&insertable) {
// //         Ok(model_bson) => match model_bson {
// //             r2d2_mongodb::mongodb::Bson::Document(model_doc) => {
// //                 match connection
// //                     .collection(COLLECTION)
// //                     .insert_one(model_doc, None)
// //                 {
// //                     Ok(res) => match res.inserted_id {
// //                         Some(res) => match bson::from_bson(res) {
// //                             Ok(res) => Ok(res),
// //                             // Err(_) => Err(ErrorKind::InternalError{message: String::from(
// //                             //     "Failed to load BSON",
// //                             // )}),            
// //                             Err(e) => return Err(mongodb::bson::de::Error::SyntaxError{message:"test".to_string()}),
// //                         },
// //                         // None => Err(ErrorKind::InternalError{message: String::from(
// //                         //     "None",
// //                         // )}),
// //                         None => return Err(mongodb::bson::de::Error::SyntaxError{message:"test".to_string()}),
// //                     },
// //                     Err(err) => Err(mongodb::bson::de::Error::SyntaxError{message:"test".to_string()}),
// //                 }
// //             }
// //             // _ => Err(ErrorKind::InternalError{message: String::from(
// //             //     "Failed to create doccument",
// //             // )}),
// //             _ => return Err(mongodb::bson::de::Error::SyntaxError{message:"test".to_string()}),
// //         },
// //         // Err(_) => Err(ErrorKind::InternalError{message: String::from(
// //         //     "Failed to create BSON",
// //         // )}),
// //         Err(_) => return Err(mongodb::bson::de::Error::SyntaxError{message:"test".to_string()}),
// //     }
// // }

// pub fn insert(cats: Cat, connection: &Conn) -> Result<ObjectId, Error> {
//     let insertable = InsertableCat::from_cat(cats.clone());
//     match bson::to_bson(&insertable) {
//         Ok(model_bson) => match model_bson {
//             bson::Bson::Document(model_doc) => {
//                 match connection
//                     .collection(COLLECTION)
//                     .insert_one(model_doc, None)
//                 {
//                     Ok(res) => match res.inserted_id {
//                         Some(res) => match bson::from_bson(res) {
//                             Ok(res) => Ok(res),
//                             Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON"))),
//                         },
//                         None => Err(Error::DefaultError(String::from("None"))),
//                     },
//                     Err(err) => Err(err),
//                 }
//             }
//             _ => Err(Error::DefaultError(String::from(
//                 "Failed to create Document",
//             ))),
//         },
//         Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
//     }
// }

// // pub fn update(id: ObjectId, cats: Cat, connection: &Conn) -> Result<Cat, ErrorKind> {
// //     let mut new_cat = cats.clone();
// //     new_cat.id = Some(id.clone());
// //     match bson::to_bson(&new_cat) {
// //         Ok(model_bson) => match model_bson {
// //             bson::Bson::Document(model_doc) => {
// //                 match connection.collection(COLLECTION).replace_one(
// //                     doc! {"_id": id},
// //                     model_doc,
// //                     None,
// //                 ) {
// //                     Ok(_) => Ok(new_cat),
// //                     Err(err) => Err(err),
// //                 }
// //             }
// //             _ => Err(ErrorKind::InternalError{message: String::from(
// //                 "Failed to create document",
// //             )}),
// //         },
// //         Err(_) => Err(ErrorKind::InternalError{message: String::from(
// //             "Failed to create BSON",
// //         )}),
// //     }
// // }

// // pub fn delete(id: ObjectId, connection: &Conn) -> Result<DeleteResult, ErrorKind> {
// //     connection
// //         .collection(COLLECTION)
// //         .delete_one(doc! {"_id": id}, None)
// // }

// // pub fn delete_all(connection: &Conn) -> Result<(), ErrorKind> {
// //     connection.collection(COLLECTION).drop()
// // }
