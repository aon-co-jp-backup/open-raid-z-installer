//! Tauriコマンド呼び出し(`window.__TAURI__.core.invoke`、tauri.conf.jsonの`withGlobalTauri`)と、
//! コマンドが返す値の型。型は`open_runo_installer_core`の直列化形式に合わせてある。

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "invoke", catch)]
    async fn invoke_raw(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// コマンドを呼ぶ。失敗時は表示用の文字列を返す。
pub async fn invoke<A: Serialize, R: DeserializeOwned>(cmd: &str, args: &A) -> Result<R, String> {
    let ser = serde_wasm_bindgen::Serializer::json_compatible();
    let js_args = args.serialize(&ser).map_err(|e| e.to_string())?;
    match invoke_raw(cmd, js_args).await {
        Ok(v) => serde_wasm_bindgen::from_value(v).map_err(|e| e.to_string()),
        Err(e) => Err(e.as_string().unwrap_or_else(|| format!("{e:?}"))),
    }
}

#[derive(Serialize)]
pub struct NoArgs {}

#[derive(Debug, Clone, Deserialize)]
pub struct DiskInfo {
    pub path: String,
    #[allow(dead_code)]
    pub index: u32,
    pub size_bytes: u64,
    pub media_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcceleratorInfo {
    pub kind: String,
    pub description: String,
    pub vendor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OsCompatEntry {
    pub os: String,
    pub status: String,
    pub note: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SystemStatus {
    pub current_os: String,
    pub os_compatibility: Vec<OsCompatEntry>,
    pub accelerators: Vec<AcceleratorInfo>,
    pub disks: Vec<DiskInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Advice {
    /// "Info" / "Suggestion" / "Warning"
    pub severity: String,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BenchmarkEntry {
    pub label: String,
    pub throughput_mb_per_sec: f64,
}

#[derive(Serialize)]
pub struct DiskSelection {
    pub path: String,
    pub size_bytes: u64,
}

pub fn gib(bytes: u64) -> String {
    format!("{:.1}", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
}
