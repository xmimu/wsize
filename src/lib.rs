use glob::glob;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

struct BaseMeta {
    size: u64,
    name: String,
    path: PathBuf,
    language: String,
    id: String,
    obj_type: String,
}

struct WemMeta {
    meta: BaseMeta,
}

struct BankMeta {
    meta: BaseMeta,
    wem_list: Vec<WemMeta>,
}

fn parse_json_data(value: &serde_json::Value) -> Vec<WemMeta> {
    let mut data: Vec<WemMeta> = Vec::new();

    if let Some(arr) = value["SoundBanksInfo"]["SoundBanks"][0]["Media"].as_array() {
        for v in arr {
            data.push(WemMeta {
                meta: BaseMeta {
                    size: 0,
                    name: v["ShortName"].as_str().unwrap().to_string(),
                    path: PathBuf::from(v["Path"].as_str().unwrap()),
                    language: v["Language"].as_str().unwrap().to_string(),
                    id: v["Id"].as_str().unwrap().to_string(),
                    obj_type: "Media".to_string(),
                },
            });
        }
    }

    data
}

fn parse_bank_data(path: &PathBuf) -> Option<BankMeta> {
    let size = path.metadata().ok()?.len();
    let name = path.file_name()?.to_str()?.to_string();
    let mut language = String::new();
    let mut id = String::new();
    let mut obj_type = String::new();
    let mut wem_list: Vec<WemMeta> = Vec::new();
    let json_file = path.with_extension("json");
    if json_file.is_file() {
        let c = fs::read_to_string(json_file).ok()?;
        let v: serde_json::Value = serde_json::from_str(&c).expect("json 文件读取失败！");
        if let Some(v_language) = v["SoundBanksInfo"]["SoundBanks"]["Language"].as_str() {
            language = v_language.to_string();
        }
        if let Some(v_id) = v["SoundBanksInfo"]["SoundBanks"]["Id"].as_str() {
            id = v_id.to_string();
        }
        if let Some(v_obj_type) = v["SoundBanksInfo"]["SoundBanks"]["Type"].as_str() {
            obj_type = v_obj_type.to_string();
        }
        wem_list = parse_json_data(&v);
    }

    Some(BankMeta {
        meta: BaseMeta {
            size: size,
            name: name,
            path: path.clone(),
            language: language,
            id: id,
            obj_type: obj_type,
        },
        wem_list: wem_list,
    })
}

#[pyfunction]
fn filter_by_name(py: Python<'_>, pattern: &str) -> PyObject {
    let paths: Vec<PathBuf> = glob(pattern)
        .expect("文件路径解析失败")
        .filter_map(Result::ok)
        .collect();

    let banks_meta: Vec<BankMeta> = paths
        .par_iter()
        .filter_map(|p| parse_bank_data(p))
        .collect();

    // 构造 PyDict
    let dict = PyDict::new(py);
    let mut banks: Vec<PyObject> = Vec::new();
    let mut total_size = 0u64;

    for i in banks_meta {
        let bank_data = PyDict::new(py);
        let mut wem_list: Vec<PyObject> = Vec::new();

        for j in i.wem_list {
            let wem_data = PyDict::new(py);
            wem_data.set_item("name", j.meta.name).unwrap();
            wem_data.set_item("size", j.meta.size).unwrap();
            wem_data.set_item("id", j.meta.id).unwrap();
            wem_data.set_item("path", j.meta.path).unwrap();
            wem_data.set_item("type", j.meta.obj_type).unwrap();
            wem_data.set_item("language", j.meta.language).unwrap();
            wem_list.push(wem_data.into());
            total_size += j.meta.size;
        }

        bank_data.set_item("name", i.meta.name).unwrap();
        bank_data.set_item("size", i.meta.size).unwrap();
        bank_data.set_item("id", i.meta.id).unwrap();
        bank_data.set_item("path", i.meta.path).unwrap();
        bank_data.set_item("type", i.meta.obj_type).unwrap();
        bank_data.set_item("language", i.meta.language).unwrap();
        bank_data.set_item("wem_list", wem_list).unwrap();
        banks.push(bank_data.into());
        total_size += i.meta.size;
    }

    dict.set_item("banks", banks).unwrap();
    dict.set_item("size", total_size).unwrap();
    dict.into()
}

/// A Python module implemented in Rust.
#[pymodule]
fn wsize(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(filter_by_name, m)?)?;
    Ok(())
}
