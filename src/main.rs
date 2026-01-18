use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber;

mod cli;
mod core;
mod models;

#[derive(Parser)]
#[command(name = "explog")]
#[command(author = "Explog Team")]
#[command(version = "0.2.0")]
#[command(about = "A blazing fast static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site
    Build {
        /// Clean build (ignore cache)
        #[arg(short, long)]
        clean: bool,
        
        /// Rebuild specific page only (by slug)
        #[arg(long)]
        page: Option<String>,
        
        /// Rebuild specific post only (by slug)
        #[arg(long)]
        post: Option<String>,
        
        /// Rebuild specific category only (by name)
        #[arg(long)]
        category: Option<String>,
        
        /// Rebuild specific tag only (by name)
        #[arg(long)]
        tag: Option<String>,
    },
    /// Start development server with hot reload
    Dev {
        /// Port to run the dev server on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Clean build artifacts
    Clean,
    /// Create a new post or page
    New {
        /// Type: post, page, or a page type (customize/embedded/standalone)
        #[arg(value_parser = ["post", "page", "customize", "embedded", "standalone"])]
        content_type: String,
        /// Title/name of the content
        title: String,
    },
    /// Plugin management commands
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
    /// Run SEO analysis and generate report
    Seo {
        /// Custom output directory (default: .seo-report)
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum PluginAction {
    /// List installed plugins
    List,
    /// Show details of a plugin
    Show {
        /// Plugin name
        name: String,
    },
    /// Create a new plugin scaffold
    New {
        /// Plugin name
        name: String,
    },
    /// Remove a plugin
    Remove {
        /// Plugin name
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Build { clean, page, post, category, tag } => {
            info!("Starting build...");
            cli::build::run(clean, page, post, category, tag).await?;
        }
        Commands::Dev { port } => {
            info!("Starting dev server on port {}...", port);
            cli::dev::run(port).await?;
        }
        Commands::Clean => {
            info!("Cleaning build artifacts...");
            cli::clean::run()?;
        }
        Commands::New { content_type, title } => {
            match content_type.as_str() {
                "post" | "page" => {
                    info!("Creating new {} : {}", content_type, title);
                    cli::new::run(&content_type, &title)?;
                }
                "customize" | "embedded" | "standalone" => {
                    info!("Creating new {} page: {}", content_type, title);
                    cli::scaffolding::create_page(&content_type, &title, "content/pages")?;
                }
                _ => {
                    anyhow::bail!("Unknown content type: {}", content_type);
                }
            }
        }
        Commands::Plugin { action } => {
            match action {
                PluginAction::List => {
                    cli::plugin::list_plugins("plugins")?;
                }
                PluginAction::Show { name } => {
                    cli::plugin::show_plugin(&name, "plugins")?;
                }
                PluginAction::New { name } => {
                    cli::plugin::new_plugin(&name, "plugins")?;
                }
                PluginAction::Remove { name } => {
                    cli::plugin::remove_plugin(&name, "plugins")?;
                }
            }
        }
        Commands::Seo { output } => {
            info!("Running SEO analysis...");
            cli::seo::run(output)?;
        }
    }

    Ok(())
}
