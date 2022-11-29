use crate::{formatting::*, util::maybe_parse_datetime};
use ad4m_client::Ad4mClient;
use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;
use rustyline::Editor;

#[derive(Args, Debug)]
pub struct QueryLinksArgs {
    /// Perspective ID
    id: String,

    /// Filter by source
    source: Option<String>,

    /// Filter by target
    target: Option<String>,

    /// Filter by predicate
    predicate: Option<String>,

    /// Get only links after this date (fromat: %Y-%m-%dT%H:%M:%S%.fZ)
    #[arg(short, long)]
    from_date: Option<String>,

    /// Get only links before this date (fromat: %Y-%m-%dT%H:%M:%S%.fZ)
    #[arg(short, long)]
    until_date: Option<String>,

    /// Get only the first n links
    #[arg(short, long)]
    limit: Option<f64>,
}

#[derive(Debug, Subcommand)]
pub enum PerspectiveFunctions {
    /// Add a perspective with given name
    Add { name: String },

    /// Remove perspective with given uuid
    Remove { id: String },

    /// Add link to perspective with given uuid
    AddLink {
        id: String,
        source: String,
        target: String,
        predicate: Option<String>,
    },

    /// Query links from perspective with given uuid
    QueryLinks(QueryLinksArgs),

    /// Retrieve snapshot of perspective with given uuid
    Snapshot { id: String },

    /// Run Prolog / SDNA query on perspective with given uuid
    Infer { id: String, query: String },

    /// Stay connected and print any changes (links added/removed) to the perspective
    Watch { id: String },

    /// Interactive Perspective shell based on Prolog/SDNA runtime
    Repl { id: String },
}

pub async fn run(ad4m_client: Ad4mClient, command: Option<PerspectiveFunctions>) -> Result<()> {
    if command.is_none() {
        let all_perspectives = ad4m_client.perspectives.all().await?;
        for perspective in all_perspectives {
            println!("\x1b[36mName: \x1b[97m{}", perspective.name);
            println!("\x1b[36mID: \x1b[97m{}", perspective.uuid);
            if perspective.shared_url.is_some() {
                println!(
                    "\x1b[36mShared URL: \x1b[97m{}",
                    perspective.shared_url.unwrap()
                );
            } else {
                println!("\x1b[36mShared URL: \x1b[90m<not shared as Neighbourhood>");
            }

            if let Some(nh) = perspective.neighbourhood {
                println!(
                    "\x1b[36mNeighbourhood Link-Language: \x1b[97m{}",
                    nh.link_language
                );
                if nh.meta.links.is_empty() {
                    println!("\x1b[36mNeughbourhood meta: \x1b[90m<empty>");
                } else {
                    println!("\x1b[36mNeughbourhood meta:");
                    for link in nh.meta.links {
                        print_link(link.into());
                    }
                }
            }
            println!()
        }

        return Ok(());
    }

    match command.unwrap() {
        PerspectiveFunctions::Add { name } => {
            let new_perspective_id = ad4m_client.perspectives.add(name).await?;
            println!("{}", new_perspective_id);
        }
        PerspectiveFunctions::Remove { id } => {
            ad4m_client.perspectives.remove(id).await?;
        }
        PerspectiveFunctions::AddLink {
            id,
            source,
            target,
            predicate,
        } => {
            ad4m_client
                .perspectives
                .add_link(id, source, target, predicate)
                .await?;
        }
        PerspectiveFunctions::QueryLinks(args) => {
            let from_date = maybe_parse_datetime(args.from_date)?;
            let until_date = maybe_parse_datetime(args.until_date)?;
            let result = ad4m_client
                .perspectives
                .query_links(
                    args.id,
                    args.source,
                    args.target,
                    args.predicate,
                    from_date,
                    until_date,
                    args.limit,
                )
                .await?;
            for link in result {
                print_link(link.into());
            }
        }
        PerspectiveFunctions::Infer { id, query } => {
            let results = ad4m_client.perspectives.infer(id, query).await?;
            print_prolog_results(results)?;
        }
        PerspectiveFunctions::Watch { id } => {
            ad4m_client
                .perspectives
                .watch(
                    id,
                    Box::new(|link| {
                        print_link(link);
                    }),
                )
                .await?;
        }
        PerspectiveFunctions::Snapshot { id } => {
            let result = ad4m_client.perspectives.snapshot(id).await?;
            println!("{:#?}", result);
        }
        PerspectiveFunctions::Repl { id } => {
            //let _ = perspectives::run_watch(cap_token, id);
            let mut rl = Editor::<()>::new()?;
            loop {
                let line = rl.readline("\x1b[97m> ")?;
                rl.add_history_entry(line.as_str());
                let line = line.trim().to_string();
                if line == "exit" {
                    break;
                }

                let add_link = Regex::new(
                    r"add-link\s+(?P<source>\S+)\s+(?P<predicate>\S+)\s+(?P<target>\S+)",
                )?;
                let caps = add_link.captures(&line);
                if let Some(caps) = caps {
                    let source = caps.name("source").unwrap().as_str().to_string();
                    let predicate = caps.name("predicate").unwrap().as_str().to_string();
                    let target = caps.name("target").unwrap().as_str().to_string();

                    let predicate = if predicate == "_" {
                        None
                    } else {
                        Some(predicate)
                    };

                    ad4m_client
                        .perspectives
                        .add_link(id.clone(), source, target, predicate)
                        .await?;
                    continue;
                }

                match ad4m_client.perspectives.infer(id.clone(), line).await {
                    Ok(results) => {
                        print_prolog_results(results)?;
                    }
                    Err(e) => {
                        println!("\x1b[91m{}", e.root_cause());
                    }
                }
            }
        }
    }
    Ok(())
}