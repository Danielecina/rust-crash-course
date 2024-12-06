use async_std::prelude::*;

// popular serialize e dese data structure in json per esempio 
use serde::{de, Serialize};
use std::error::Error;
use std::marker::Unpin;

// creiamo un nuovo nome (ChatError) per un tipo esistente, in modo che il codice sia più chiaro e riutilizzabile.
// Box<dyn Error + Send + Sync + 'static>: Questa è la parte più complessa
// Box: È un tipo smart pointer che permette l’allocazione dinamica in heap.
// Sappiamo che Box Viene usato per allocare oggetti dinamici a runtime, evitando di dover specificare la dimensione dell’oggetto a compile time
// dyn Error: Questo indica che Box conterrà un oggetto che implementa il trait Error (polimorfismo dinamico)
// Send + Sync indica che: 
// che il tipo può essere trasferito tra thread in modo sicuro; 
// che il tipo può essere condiviso tra thread in modo sicuro;
// 'static: è un lifetime bound che indica che il tipo vivrà per tutta la durata del programma
pub type ChatError = Box<dyn Error + Send + Sync + 'static>; 
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
  O: async_std::io::Write + Unpin,
  P: Serialize, 
{
  let mut json = serde_json::to_string(&packet)?;
  json.push('\n');

  leaving.write_all(json.as_bytes()).await?;
  Ok(())
}

pub fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
  I: async_std::io::BufRead + Unpin,
  T: de::DeserializeOwned,
{
  incoming.lines().map(|line| -> ChatResult<T> {
    let li = line?;
    let msg = serde_json::from_str::<T>(&li)?;
    Ok(msg)
  })
}