// Copyright 2023 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{anyhow, bail, Context};
use cargo_edit::LocalManifest;
use cargo_generate::{generate, GenerateArgs, TemplatePath};
use clap::Parser;
use reqwest::header::{HeaderMap, CONTENT_TYPE, COOKIE, USER_AGENT};
use reqwest::redirect::Policy;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use toml_edit::{InlineTable, Value};

/// Simple Advent of Code template initialiser.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specifies the event year
    #[arg(short, long, required = true)]
    year: String,

    /// Specifies the event day
    #[arg(short, long, required = true)]
    day: String,

    /// Location of the cargo-generate template
    #[arg(long)]
    workspace_root: Option<PathBuf>,
}

fn generate_project_files(args: &Args, root: &Path) -> anyhow::Result<()> {
    let year = &args.year;
    let day = &args.day;

    // TODO: assumption: the command is run from the workspace root
    let template_path = root.join("tools/aoc-init/template");

    let generate_args = GenerateArgs {
        template_path: TemplatePath {
            path: Some(template_path.display().to_string()),
            ..Default::default()
        },
        name: Some("placeholder".to_string()),
        vcs: None,
        init: true,
        define: vec![format!("year={year}"), format!("day={day}")],
        ..Default::default()
    };

    generate(generate_args)?;

    Ok(())
}

fn add_to_workspace(args: &Args, root: &Path) -> anyhow::Result<()> {
    let package_path = format!("{}/day{}", args.year, args.day);
    let mut manifest = LocalManifest::find(Some(root))?;
    let members = manifest
        .manifest
        .data
        .get_mut("workspace")
        .context("no workspace")?
        .get_mut("members")
        .context("no members")?
        .as_array_mut()
        .context("members are not an array")?;

    for member in members.iter() {
        let member_str = member
            .as_str()
            .context("workspace member is not a string")?;

        if package_path == member_str {
            bail!("{package_path} is already part of the workspace")
        }
    }

    // update the formatting info
    let first_decor = members.iter().next().unwrap().decor().clone();
    let last_decor = members.iter().last().unwrap().decor();

    let mut new_entry = toml_edit::Value::from(package_path);
    let new_entry_decor = new_entry.decor_mut();
    if let Some(prefix) = last_decor.prefix() {
        new_entry_decor.set_prefix(prefix.clone())
    }
    if let Some(suffix) = last_decor.suffix() {
        new_entry_decor.set_suffix(suffix.clone())
    }

    let last_member = members.iter_mut().last().unwrap();
    let old_last_decor = last_member.decor_mut();
    if let Some(prefix) = first_decor.prefix() {
        old_last_decor.set_prefix(prefix.clone())
    }
    if let Some(suffix) = first_decor.suffix() {
        old_last_decor.set_suffix(suffix.clone())
    }

    members.push_formatted(new_entry);
    manifest.write()?;

    Ok(())
}

fn add_to_solution_runner(args: &Args, root: &Path) -> anyhow::Result<()> {
    let year = &args.year;
    let day = &args.day;
    let day_normalised: u8 = day.parse()?;
    let package_name = format!("day{day}_{year}");
    let package_path = format!("../{year}/day{day}");

    let mut manifest = LocalManifest::find(Some(root.join("solution-runner").as_path()))?;
    let runner_main = root.join("solution-runner").join("src").join("main.rs");
    let dependencies = manifest
        .manifest
        .data
        .get_mut("dependencies")
        .context("no dependencies")?
        .as_table_like_mut()
        .context("dependencies are not a table")?;

    let mut aoc_deps = Vec::new();

    for (name, _) in dependencies.iter() {
        if name == package_name {
            bail!("{package_name} is already in the dependencies of the solution runner")
        }
        if name.starts_with("day") {
            aoc_deps.push(name)
        }
    }

    let mut table = InlineTable::new();
    table.insert("path", package_path.into());

    dependencies.insert(&package_name, toml_edit::value(Value::InlineTable(table)));
    manifest.write()?;

    // is this extremely naive and fragile?
    // yes.
    // does it work (for now?)
    // sure.
    let content = fs::read_to_string(&runner_main)?;
    let mut content_lines = content.lines().collect::<Vec<_>>();

    let end_index = content_lines
        .iter()
        .enumerate()
        .rfind(|(_, l)| l.contains("AUTOGENERATED SOLUTIONS END"))
        .map(|(i, _)| i)
        .ok_or(anyhow!("AUTOGENERATED tags missing"))?;

    let def = format!(
        "    define_solution!(args, {year}, {day_normalised}, \"inputs/{year}/day{day}\", {package_name}::Day{day});"
    );
    content_lines.insert(end_index, &def);

    let new_content = content_lines.join("\n");
    fs::write(&runner_main, new_content)?;

    Ok(())
}

fn try_get_input(args: &Args, root: &Path) -> anyhow::Result<()> {
    let year = &args.year;
    let day = &args.day;
    let day_normalised: u8 = day.parse()?;

    let input_file = root.join("inputs").join(year).join(format!("day{day}"));

    let mut file = fs::File::create(input_file)?;

    let Ok(session_cookie) = env::var("AOC_SESSION") else {
        return Ok(());
    };

    let cookie_header = format!("session={}", session_cookie.trim());

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header.parse()?);
    headers.insert(CONTENT_TYPE, "text/plain".parse()?);
    headers.insert(
        USER_AGENT,
        "https://github.com/jstuczyn/AdventOfCode by jedrzej.stuczynski@gmail.com".parse()?,
    );

    let input = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()?
        .get(format!(
            "https://adventofcode.com/{year}/day/{day_normalised}/input"
        ))
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.bytes())?;

    file.write_all(&input)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let root = args.workspace_root.clone().unwrap_or(env::current_dir()?);

    generate_project_files(&args, &root)?;
    add_to_workspace(&args, &root)?;
    add_to_solution_runner(&args, &root)?;
    try_get_input(&args, &root)?;

    Ok(())
}
