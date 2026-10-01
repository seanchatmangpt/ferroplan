//! Drift court: `qri:freeArity` in the contract must equal the parameter count of the
//! exported `fp_dealloc` in `src/wasi_abi.rs`, and the generated host must call the free
//! export with exactly that many arguments. Pure functions over text; the mutation
//! self-tests prove each check refuses.

use std::path::PathBuf;

fn read(rel: &str) -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
}

/// The single `qri:freeArity N` value of a contract.
fn contract_free_arity(ttl: &str) -> Result<u32, String> {
    let vals: Vec<u32> = ttl
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .flat_map(|l| {
            l.split("qri:freeArity")
                .skip(1)
                .filter_map(|rest| rest.split_whitespace().next())
                .map(|t| t.trim_end_matches([';', '.', ',']).parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect::<Result<_, _>>()
        .map_err(|e| format!("unparsable qri:freeArity: {e}"))?;
    match vals.as_slice() {
        [v] => Ok(*v),
        other => Err(format!(
            "expected exactly one qri:freeArity, found {}",
            other.len()
        )),
    }
}

/// Parameter count of `pub unsafe extern "C" fn fp_dealloc(...)`.
fn dealloc_param_count(src: &str) -> Result<u32, String> {
    let start = src
        .find("fn fp_dealloc(")
        .ok_or("fn fp_dealloc( not found")?
        + "fn fp_dealloc(".len();
    let end = start
        + src[start..]
            .find(')')
            .ok_or("fp_dealloc signature unterminated")?;
    let params = src[start..end].trim();
    if params.is_empty() {
        return Ok(0);
    }
    Ok(params.split(',').filter(|p| !p.trim().is_empty()).count() as u32)
}

/// Argument counts of every `call_raw(state, "<sym>", [..]` in generated host.ex.
fn generated_free_call_arities(host: &str, sym: &str) -> Vec<u32> {
    let needle = format!("call_raw(state, \"{sym}\", [");
    host.match_indices(&needle)
        .map(|(i, _)| {
            let rest = &host[i + needle.len()..];
            let list = &rest[..rest.find(']').unwrap_or(rest.len())];
            list.split(',').filter(|a| !a.trim().is_empty()).count() as u32
        })
        .collect()
}

fn check(ttl: &str, abi: &str, host: &str) -> Result<(), String> {
    let declared = contract_free_arity(ttl)?;
    let actual = dealloc_param_count(abi)?;
    if declared != actual {
        return Err(format!(
            "contract qri:freeArity {declared} != fp_dealloc params {actual}"
        ));
    }
    let calls = generated_free_call_arities(host, "fp_dealloc");
    if calls.is_empty() {
        return Err("generated host.ex has no fp_dealloc call".into());
    }
    if let Some(bad) = calls.iter().find(|n| **n != declared) {
        return Err(format!(
            "generated host.ex calls fp_dealloc with {bad} args, freeArity is {declared}"
        ));
    }
    Ok(())
}

fn inputs() -> (String, String, String) {
    (
        read("ontology/contract.ttl"),
        read("src/wasi_abi.rs"),
        read("generated/beam-host/host.ex"),
    )
}

#[test]
fn free_arity_matches_abi_and_generated_host() {
    let (ttl, abi, host) = inputs();
    check(&ttl, &abi, &host).unwrap();
    assert_eq!(
        dealloc_param_count(&abi).unwrap(),
        2,
        "live ABI is fp_dealloc(ptr, len)"
    );
}

#[test]
fn template_consumes_free_arity() {
    let t = read("beam-host-templates/host.ex.tmpl");
    assert!(
        t.contains("qri:freeArity") && t.contains("p.free_arity"),
        "host.ex.tmpl ignores qri:freeArity"
    );
}

#[test]
fn refuses_contract_arity_mutation() {
    let (ttl, abi, host) = inputs();
    let m = ttl.replace("qri:freeArity 2", "qri:freeArity 1");
    assert_ne!(m, ttl);
    assert!(check(&m, &abi, &host).is_err());
}

#[test]
fn refuses_abi_param_mutation() {
    let (ttl, abi, host) = inputs();
    let m = abi.replace(
        "fn fp_dealloc(ptr: *mut u8, len: usize)",
        "fn fp_dealloc(ptr: *mut u8)",
    );
    assert_ne!(m, abi);
    assert!(check(&ttl, &m, &host).is_err());
}

#[test]
fn refuses_generated_host_arity_mutation() {
    let (ttl, abi, host) = inputs();
    let m = host.replace("[out_ptr, out_len]", "[out_ptr]");
    assert_ne!(m, host);
    assert!(check(&ttl, &abi, &m).is_err());
}

#[test]
fn refuses_missing_arity() {
    let (ttl, abi, host) = inputs();
    let m = ttl.replace("qri:freeArity 2 ;", "");
    assert_ne!(m, ttl);
    assert!(check(&m, &abi, &host).is_err());
}
