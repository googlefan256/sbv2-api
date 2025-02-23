use std::io::{Cursor, Read};

use tar::Archive;
use zstd::decode_all;

use crate::error::{Error, Result};

/// Parse a .sbv2 file binary
///
/// # Examples
///
/// ```rs
/// parse_sbv2file("tsukuyomi", std::fs::read("tsukuyomi.sbv2")?)?;
/// ```
pub fn parse_sbv2file<P: AsRef<[u8]>>(sbv2_bytes: P) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut arc = Archive::new(Cursor::new(decode_all(Cursor::new(sbv2_bytes.as_ref()))?));
    let mut vits2 = None;
    let mut style_vectors = None;
    let mut et = arc.entries()?;
    while let Some(Ok(mut e)) = et.next() {
        let pth = String::from_utf8_lossy(&e.path_bytes()).to_string();
        let mut b = Vec::with_capacity(e.size() as usize);
        e.read_to_end(&mut b)?;
        match pth.as_str() {
            "model.onnx" => vits2 = Some(b),
            "style_vectors.json" => style_vectors = Some(b),
            _ => continue,
        }
    }
    if style_vectors.is_none() {
        return Err(Error::ModelNotFoundError("style_vectors".to_string()));
    }
    if vits2.is_none() {
        return Err(Error::ModelNotFoundError("vits2".to_string()));
    }
    Ok((style_vectors.unwrap(), vits2.unwrap()))
}

/// Parse a .aivmx file binary
///
/// # Examples
///
/// ```rs
/// parse_aivmx("tsukuyomi", std::fs::read("tsukuyomi.aivmx")?)?;
/// ```
#[cfg(feature = "aivmx")]
pub fn parse_aivmx<P: AsRef<[u8]>>(aivmx_bytes: P) -> Result<(Vec<u8>, Vec<u8>)> {
    use base64::{prelude::BASE64_STANDARD, Engine};
    use ndarray::ShapeBuilder;

    use crate::style::Data;
    let model = crate::model::load_model(&aivmx_bytes, false)?;
    let metadata = model.metadata()?;
    if let Some(aivm_style_vectors) = metadata.custom("aivm_style_vectors")? {
        let aivm_style_vectors = BASE64_STANDARD.decode(aivm_style_vectors)?;
        let style_vectors = Cursor::new(&aivm_style_vectors);
        let reader = npyz::NpyFile::new(style_vectors)?;
        let (data, shape) = {
            let shape = reader.shape().to_vec();
            let order = reader.order();
            let data = reader.into_vec::<f32>()?;
            let shape = match shape[..] {
                [i1, i2] => [i1 as usize, i2 as usize],
                _ => return Err(Error::ModelNotFoundError("expected 2D array".to_string())),
            };
            let true_shape = shape.set_f(order == npyz::Order::Fortran);
            let v = ndarray::Array2::from_shape_vec(true_shape, data)?;
            (
                v.outer_iter().map(|row| row.to_vec()).collect(),
                v.shape().to_vec().try_into().unwrap(),
            )
        };
        drop(metadata);
        Ok((
            serde_json::to_vec(&Data { shape, data })?,
            aivmx_bytes.as_ref().to_vec(),
        ))
    } else {
        Err(Error::ModelNotFoundError("Invalid aivmx file".to_string()))
    }
}
