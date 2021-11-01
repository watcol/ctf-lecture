mod toml;

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::toml::List;
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
            message: gen_html(list.description)?,
            problems: list
                .problems
                .into_iter()
                .enumerate()
                .map(|(i, pro)| {
                    Ok(Problem {
                        count: i + 1,
                        name: pro.name.clone(),
                        title: pro.title,
                        digest: pro.digest,
                        includes: pro.includes,
                        message: gen_html(pro.message)?,
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
    pub name: String,
    pub title: String,
    pub digest: String,
    pub includes: Vec<PathBuf>,
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
        let dst = format!("webroot/{}", pro.name);
        let dst = Path::new(&dst);
        std::fs::create_dir(dst)?;
        for i in pro.includes.iter() {
            std::fs::copy(Path::new("resource").join(i), dst.join(i))?;
        }
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

fn gen_html(input: String) -> anyhow::Result<String> {
    let parser = Parser::new(&input);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    Ok(html)
}
