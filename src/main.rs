use serde::{ Deserialize, Serialize };
use clap::{ Parser, Subcommand };
use std::collections::HashMap;
use rss::Channel;
use macro_colors::*;

#[derive(Debug, Parser)]
#[command(name = "quibbler")]
#[command(version = "1.0")]
#[command(author = "Todd Martin")]
struct Cli {
		#[command(subcommand)]
		command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
		/// Add a RSS feed
		Add {
				name: String,
				url: String,
		},
		/// Fetch all feeds
		Fetch { },
		/// List all feeds
		List { },
		/// Remove a feed by name
		Remove { name: String },
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Configuration {
		feeds: HashMap<String, String>
}

async fn fetch_feed(url: String) -> Result<Channel, anyhow::Error> {
		let content = reqwest::get(url)
				.await?
				.bytes()
				.await?;
		
		let channel = Channel::read_from(&content[..])?;

		Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
		let mut cfg: Configuration = confy::load("quibbler", "quibbler")?;
		
		let cli = Cli::parse();

		match &cli.command {
				Some(Commands::Fetch {}) => {
						for (key, value) in cfg.feeds.clone().into_iter() {
								bold_green_println!("{} - {}", key, value);
								let channel = fetch_feed(value).await?;
								for item in channel.items.into_iter().map(|item| item) {
										if let Some(title) = item.title {
												println!("\t{}", title);
										}

										if let Some(author) = item.author {
												println!("\t\t{}", author);
										}

										if let Some(date) = item.pub_date {
												println!("\t\t{}", date);
										}

										if let Some(link) = item.link {
												italic_blue_println!("\t\t{}", link);
										}
								}
						}
				},
				Some(Commands::Add { name, url }) =>  { cfg.feeds.insert(name.clone(), url.clone()); },
				Some(Commands::List {}) =>  {
						for (key, value) in cfg.feeds.clone().into_iter() {
								println!("{} - {}", key, value);
						}
				},
				Some(Commands::Remove { name }) => {
						cfg.feeds.remove(name);
				},
				None => { },
		};

		confy::store("quibbler", "quibbler", cfg)?;
		
		Ok(())
}
