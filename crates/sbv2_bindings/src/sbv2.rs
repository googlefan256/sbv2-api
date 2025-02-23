use pyo3::prelude::*;
use pyo3::types::PyBytes;
use sbv2_core::tts::{SynthesizeOptions, TTSModelHolder};

use std::fs;

#[pyclass]
pub struct TTSModel {
    pub model: TTSModelHolder,
}

#[pymethods]
impl TTSModel {
    #[pyo3(signature = (bert_model_bytes, tokenizer_bytes, max_loaded_models=None))]
    #[new]
    fn new(
        bert_model_bytes: Vec<u8>,
        tokenizer_bytes: Vec<u8>,
        max_loaded_models: Option<usize>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            model: TTSModelHolder::new(bert_model_bytes, tokenizer_bytes, max_loaded_models)?,
        })
    }

    #[pyo3(signature = (bert_model_path, tokenizer_path, max_loaded_models=None))]
    #[staticmethod]
    fn from_path(
        bert_model_path: String,
        tokenizer_path: String,
        max_loaded_models: Option<usize>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            model: TTSModelHolder::new(
                fs::read(bert_model_path)?,
                fs::read(tokenizer_path)?,
                max_loaded_models,
            )?,
        })
    }

    fn load_sbv2file(&mut self, ident: String, sbv2file_bytes: Vec<u8>) -> anyhow::Result<()> {
        self.model.load_sbv2file(ident, sbv2file_bytes)?;
        Ok(())
    }

    fn load_aivmx(&mut self, ident: String, aivmx_bytes: Vec<u8>) -> anyhow::Result<()> {
        self.model.load_aivmx(ident, aivmx_bytes)?;
        Ok(())
    }

    fn load_sbv2file_from_path(
        &mut self,
        ident: String,
        sbv2file_path: String,
    ) -> anyhow::Result<()> {
        self.model.load_sbv2file(ident, fs::read(sbv2file_path)?)?;
        Ok(())
    }

    fn load_aivmx_from_path(&mut self, ident: String, aivmx_path: String) -> anyhow::Result<()> {
        self.model.load_aivmx(ident, fs::read(aivmx_path)?)?;
        Ok(())
    }

    fn synthesize<'p>(
        &'p mut self,
        py: Python<'p>,
        text: String,
        ident: String,
        style_id: i32,
        speaker_id: i64,
        sdp_ratio: f32,
        length_scale: f32,
    ) -> anyhow::Result<Bound<'p, PyBytes>> {
        let data = self.model.easy_synthesize(
            ident.as_str(),
            &text,
            style_id,
            speaker_id,
            SynthesizeOptions {
                sdp_ratio,
                length_scale,
                ..Default::default()
            },
        )?;
        Ok(PyBytes::new(py, &data))
    }

    fn unload(&mut self, ident: String) -> bool {
        self.model.unload(ident)
    }
}
