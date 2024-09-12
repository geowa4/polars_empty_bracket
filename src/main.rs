use std::sync::Arc;

use polars::prelude::{DataType, Field, LazyFileListReader, LazyJsonLineReader, Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = "empty_brackets.ndjson";

    let schema = Arc::new(Schema::from_iter(vec![
        Field::new("foo", DataType::String)
    ]));

    let df = LazyJsonLineReader::new(file)
        .with_schema(Some(schema))
        .finish()?
        .collect()?;

    println!("{:?}", df);

    Ok(())
}
