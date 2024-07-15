use std::collections::HashMap;

use arrow_schema::{DataType, Field, Schema};
use petgraph::graph::UnGraph;

use mems::model::{get_csv_str, get_island_from_plugin_input, get_wasm_result, ModelType, PluginInput, PluginOutput};
use mems::model::dev::{CN, Island, PropDefine, PsRsrType, RsrDefine};

// use std::fs;
// use std::io::Write;
// use std::path::PathBuf;

const NORMAL_OPEN: &str = "normalOpen";
const STATIC_TOPO_DF_NAME: &str = "static_topo";
const TERMINAL_DF_NAME: &str = "terminal_cn_dev";
const POINT_DF_NAME: &str = "point_terminal_phase";

#[no_mangle]
pub unsafe fn run(ptr: i32, len: u32) -> u64 {
    // 从内存中获取字符串
    let input = unsafe {
        let slice = std::slice::from_raw_parts(ptr as _, len as _);
        let input: PluginInput = serde_cbor::from_slice(slice).unwrap();
        input
    };
    let mut error = None;
    let r = get_island_from_plugin_input(&input);
    if let Err(s) = &r {
        error = Some(s.clone());
    }
    if error.is_none() {
        let (island, prop_defs, defines) = r.unwrap();
        let mut outgoing = vec![];
        for model_input in &input.model {
            match model_input {
                ModelType::Outgoing(edge_name) => {
                    outgoing = edge_name.clone();
                }
                _ => {}
            }
        }
        let mut csv_bytes = vec![];
        let mut schema = vec![];
        // create file
        // let mut base = PathBuf::from("/");
        // base.push("static_graph.csv");
        // let mut file = fs::OpenOptions::new()
        //     .create(true)
        //     .write(true)
        //     .truncate(true)
        //     .open(&base)
        //     .expect("Could not create file");
        // write graph
        let mut is_matched = false;
        if outgoing.is_empty() || outgoing.contains(&STATIC_TOPO_DF_NAME.to_string()) {
            is_matched = true;
            create_static_topo(&island, &prop_defs, &defines, &mut csv_bytes, &mut schema);
        }
        if outgoing.contains(&TERMINAL_DF_NAME.to_string()) {
            is_matched = true;
            let mut terminal_csv_str = String::from("terminal,cn,dev\n");
            let mut terminal_to_cn = HashMap::with_capacity(2 * island.cns.len());
            // 先建立CN对应的节点
            for cn in &island.cns {
                for terminal in &cn.terminals {
                    terminal_to_cn.insert(*terminal, cn.id);
                }
            }
            for (id, dev) in &island.resources {
                for terminal in &dev.terminals {
                    let terminal_id = terminal.id;
                    if let Some(cn_id) = terminal_to_cn.get(&terminal_id) {
                        terminal_csv_str.push_str(format!("{terminal_id},{cn_id},{id}\n").as_str());
                    }
                }
            }
            csv_bytes.push((TERMINAL_DF_NAME.to_string(), terminal_csv_str.into_bytes()));
            schema.push(Schema::new(vec![
                Field::new("terminal", DataType::UInt64, false),
                Field::new("cn", DataType::UInt64, false),
                Field::new("dev", DataType::UInt64, false),
            ]));
        }
        // if let Err(e) = file.write_all(csv_str.as_bytes()) {
        //     log::warn!("!!Failed to write file, err: {:?}", e);
        // } else {
        //     let _ = file.sync_all();
        // }
        if outgoing.contains(&POINT_DF_NAME.to_string()) {
            is_matched = true;
            let mut point_csv_str = String::from("point,terminal,phase\n");
            for (_, defines) in &island.measures {
                for def in defines {
                    let point_id = def.point_id;
                    let terminal_id = def.terminal_id;
                    let phase = def.phase.to_string();
                    point_csv_str.push_str(&format!("{point_id},{terminal_id},{phase}\n"))
                }
            }
            csv_bytes.push((POINT_DF_NAME.to_string(), point_csv_str.into_bytes()));
            schema.push(Schema::new(vec![
                Field::new("point", DataType::UInt64, false),
                Field::new("terminal", DataType::UInt64, false),
                Field::new("phase", DataType::Utf8, false),
            ]));
        }
        // if not matched, default is used
        if !is_matched {
            create_static_topo(&island, &prop_defs, &defines, &mut csv_bytes, &mut schema);
        }
        let output = PluginOutput {
            error_msg: None,
            schema: Some(schema),
            csv_bytes,
        };
        get_wasm_result(output)
    } else {
        let output = PluginOutput {
            error_msg: error,
            schema: None,
            csv_bytes: vec![],
        };
        get_wasm_result(output)
    }
}

fn create_static_topo(island: &Island, prop_defs: &[PropDefine], defines: &HashMap<u64, RsrDefine>, csv_bytes: &mut Vec<(String, Vec<u8>)>, schema: &mut Vec<Schema>) {
    let mut topo_csv_str = String::from("source,target,id,type,open,name\n");
    // build node_switch_model
    let mut ori_graph: UnGraph<CN, u64> = UnGraph::new_undirected();
    let mut terminal_to_idx = HashMap::with_capacity(2 * island.cns.len());
    // 先建立CN对应的节点
    for cn in &island.cns {
        let index = ori_graph.add_node(cn.clone());
        for terminal in &cn.terminals {
            terminal_to_idx.insert(*terminal, index);
        }
    }
    // 建立有两个terminal设备形成的边
    for (id, dev) in &island.resources {
        if dev.terminals.len() != 2 {
            continue;
        }
        if let Some(cn1) = terminal_to_idx.get(&dev.terminals[0].id) {
            if let Some(cn2) = terminal_to_idx.get(&dev.terminals[1].id) {
                ori_graph.add_edge(*cn1, *cn2, *id);
            }
        }
    }
    let mut prop_defines = HashMap::with_capacity(prop_defs.len());
    for def in prop_defs.into_iter() {
        prop_defines.insert(def.id, def);
    }
    for edge in ori_graph.raw_edges() {
        let s = edge.source();
        let t = edge.target();
        let cn1 = ori_graph.node_weight(s);
        let cn2 = ori_graph.node_weight(t);
        if cn1.is_none() || cn2.is_none() {
            log::warn!("!!Failed to find nodes for edge {}", edge.weight);
            topo_csv_str = String::from("source,target,id,type,name\n");
            break;
        }
        let id1 = cn1.unwrap().id;
        let id2 = cn1.unwrap().id;
        let dev_id = edge.weight;
        let mut dev_name = "".to_string();
        let mut dev_type = 0u16;
        let mut normal_open = false;
        if let Some(rsr) = island.resources.get(&dev_id) {
            dev_name = get_csv_str(&rsr.name);
            if let Some(def) = defines.get(&rsr.define_id) {
                dev_type = def.rsr_type as u16;
                if def.rsr_type == PsRsrType::Switch {
                    let v = rsr.get_prop_value2(NORMAL_OPEN, &island.prop_groups, &prop_defines);
                    if let Some(b) = v.get_bool() {
                        normal_open = b;
                    }
                }
            }
        }
        topo_csv_str.push_str(format!("{id1},{id2},{dev_id},{dev_type},{normal_open},{dev_name}\n").as_str());
    }
    csv_bytes.push((STATIC_TOPO_DF_NAME.to_string(), topo_csv_str.into_bytes()));
    schema.push(Schema::new(vec![
        Field::new("source", DataType::UInt64, false),
        Field::new("target", DataType::UInt64, false),
        Field::new("id", DataType::UInt64, false),
        // if using uint16, will get: unsupported data type when reading CSV: u16
        Field::new("type", DataType::UInt32, false),
        Field::new("open", DataType::Boolean, false),
        Field::new("name", DataType::Utf8, true),
    ]));
}