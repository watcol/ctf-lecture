mod toml;

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::toml::List;
use anyhow::Context;
use fs_extra::dir::CopyOptions;
use pulldown_cmark::{html, Parser};
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone)]
#[template(path = "index.stpl")]
struct Index {
    pub title: String,
    pub message: String,
    pub problems: Vec<Problem>,
}

impl Index {
    fn from_list(list: List) -> anyhow::Result<Self> {
        Ok(Self {
            title: list.title,
            message: gen_html("resource/description.md")?,
            problems: list
                .problems
                .into_iter()
                .enumerate()
                .map(|(i, pro)| {
                    Ok(Problem {
                        count: i + 1,
                        title: pro,
                        message: gen_html(format!("resource/{:02}/message.md", i + 1))?,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        })
    }
}

#[derive(TemplateOnce, Clone)]
#[template(path = "problem.stpl")]
struct Problem {
    pub count: usize,
    pub title: String,
    pub message: String,
}

fn main() -> anyhow::Result<()> {
    let list = toml::List::from_toml("resource/list.toml")?;
    let index = Index::from_list(list)?;
    let problems = index.problems.clone();
    let webroot = Path::new("webroot");
    if webroot.exists() {
        std::fs::remove_dir_all(webroot)?;
    }
    std::fs::create_dir("webroot")?;
    write("webroot/index.html", index)?;
    for pro in problems {
        let src = format!("resource/{:02}", pro.count);
        let dst = format!("webroot/{:02}", pro.count);
        let dst = Path::new(&dst);
        fs_extra::dir::copy(
            src,
            dst.parent().context("Root Directory")?,
            &CopyOptions::new(),
        )?;
        write(dst.join("index.html"), pro)?;
    }
    Ok(())
}

fn write<P: AsRef<Path>, C: TemplateOnce>(path: P, ctx: C) -> anyhow::Result<()> {
    let path = path.as_ref();
    let mut file = File::create(path)?;
    writeln!(file, "{}", ctx.render_once()?)?;
    Ok(())
}

fn gen_html<P: AsRef<Path>>(md: P) -> anyhow::Result<String> {
    let input = {
        let mut file = File::open(md)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        s
    };
    let parser = Parser::new(&input);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    Ok(html)
}
