use std::{process::Command, rc::Rc};

use regex::Regex;

use crate::{
    err::Result as RmusResult,
    opt::{MatchMode, Opt},
};

pub fn get_files(opt: &Opt) -> RmusResult<Vec<Rc<str>>> {
    let mut exprs = opt
        .expressions
        .iter()
        .map(|exp| match opt.case_insensitive {
            true => Regex::new(&format!("(?i){}", exp)),
            false => Regex::new(exp),
        });

    let base_expr = match opt.all {
        true => exprs.next().unwrap_or(Regex::new(r".*")),
        false => Regex::new(r"\.(mp3|ogg|wav)$"),
    }?;

    let exprs = exprs.collect::<Result<Vec<_>, _>>()?;

    Ok(locate(base_expr)?
        .lines()
        .map(Rc::from)
        .filter(|file| matches_in_opt(&exprs, file, opt))
        .collect())
}

fn matches_in_opt(exprs: &[Regex], file: &Rc<str>, opt: &Opt) -> bool {
    exprs
        .iter()
        .map(|expr| expr.is_match(file))
        .reduce(|a, b| match opt.matchmode {
            MatchMode::All => a && b,
            MatchMode::Any => a || b,
        })
        .unwrap_or(true)
}

fn locate(regex: Regex) -> RmusResult<String> {
    let raw_output = Command::new("locate")
        .args(["--regex", &regex.to_string()])
        .output()?
        .stdout;
    let output = String::from_utf8(raw_output)?;
    Ok(output)
}
