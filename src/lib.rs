#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::{path::PathBuf};

use bm42::{BM42Options, BM42};

const DEFAULT_CACHE_DIR: &str = ".bm42_cache";
const DEFAULT_ALPHA: f64 = 0.5;
const DEFAULT_SHOW_DOWNLOAD_PROGRESS: bool = true;

#[napi(js_name = "BM42")]
pub struct JsBM42 {
  bm42: BM42,
}

#[napi(object)]
pub struct JsBM42Options {
  pub alpha: Option<f64>,
  pub cache_dir: Option<String>,
  pub show_download_progress: Option<bool>,
}

#[napi(object)]
pub struct JsSparseEmbedding {
  pub indices: Vec<i32>,
  pub values: Vec<f64>,
}

impl Default for JsBM42Options {
  fn default() -> Self {
    JsBM42Options {
      alpha: Some(DEFAULT_ALPHA),
      cache_dir: None,
      show_download_progress: Some(DEFAULT_SHOW_DOWNLOAD_PROGRESS),
    }
  }
}

impl From<JsBM42Options> for BM42Options {
  fn from(model: JsBM42Options) -> Self {
    BM42Options {
      alpha: model.alpha.unwrap_or(DEFAULT_ALPHA) as f32,
      cache_dir: PathBuf::from(model.cache_dir.unwrap_or(DEFAULT_CACHE_DIR.to_string())),
      show_download_progress: model
        .show_download_progress
        .unwrap_or(DEFAULT_SHOW_DOWNLOAD_PROGRESS),
      ..Default::default()
    }
  }
}

#[napi]
impl JsBM42 {
  #[napi(constructor)]
  pub fn new(opts: Option<JsBM42Options>) -> Self {
    let options: BM42Options = opts.unwrap_or_default().into();

    JsBM42 {
      bm42: BM42::try_new(options).unwrap(),
    }
  }

  #[napi]
  pub fn embed(&self, texts: Vec<&str>) -> Vec<JsSparseEmbedding> {
    let results = self.bm42.embed(texts).unwrap();

    results
      .into_iter()
      .map(|embedding| JsSparseEmbedding {
        indices: embedding.indices,
        values: embedding.values.iter().map(|v| *v as f64).collect(),
      })
      .collect()
  }

  #[napi]
  pub fn query_embed(&self, texts: Vec<&str>) -> Vec<JsSparseEmbedding> {
    let results = self.bm42.query_embed(texts).unwrap();

    results
      .into_iter()
      .map(|embedding| JsSparseEmbedding {
        indices: embedding.indices,
        values: embedding.values.iter().map(|v| *v as f64).collect(),
      })
      .collect()
  }
}