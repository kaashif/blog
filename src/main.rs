// Copyright (c) 2013-2026 Kaashif Hymabaccus
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR
// IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::ffi::{c_char, c_int, c_void};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Stdio};
use std::sync::Mutex;

use anyhow::{Context, Result, anyhow, bail};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use rayon::prelude::*;
use serde::Deserialize;
use tiny_http::{Header, Response, Server, StatusCode};
use walkdir::WalkDir;

const DATA_ROOT: &str = "/usr/local/share/tau";
const OUTPUT_DIR: &str = "site";
const STAGING_DIR: &str = ".tau-site-new";
const DISCOUNT_FLAGS: u32 = 0x0000_0004 | 0x0001_0000 | 0x0100_0000 | 0x0200_0000;

static DISCOUNT_LOCK: Mutex<()> = Mutex::new(());

unsafe extern "C" {
    fn mkd_string(text: *const c_char, length: c_int, flags: u32) -> *mut c_void;
    fn mkd_compile(document: *mut c_void, flags: u32) -> c_int;
    fn mkd_document(document: *mut c_void, output: *mut *mut c_char) -> c_int;
    fn mkd_cleanup(document: *mut c_void);
}

#[derive(Parser)]
#[command(name = "tau", about = "Generate kaashif's blog")]
struct Cli {
    /// Change to DIR before executing the command.
    #[arg(short = 'r', long = "root", default_value = ".", global = true)]
    root: PathBuf,

    /// Print extra progress information.
    #[arg(short = 'v', long = "verbose", global = true)]
    _verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Regenerate the site/ directory.
    Regen,
    /// Initialise the current directory as a default site.
    Init,
    /// Serve site/ on http://localhost:8080.
    Serve,
    /// Run uploadcmd from config.yaml.
    Upload,
}

#[derive(Debug, Deserialize)]
struct Config {
    site: SiteConfig,
}

#[derive(Debug, Deserialize)]
struct SiteConfig {
    title: String,
    author: String,
    tagline: String,
    url: String,
    uploadcmd: String,
    #[serde(default)]
    extrafiles: Vec<PathBuf>,
}

#[derive(Debug)]
struct Post {
    title: String,
    date: String,
    content: String,
    link: String,
}

struct Templates {
    default: String,
    post: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    std::env::set_current_dir(&cli.root)
        .with_context(|| format!("changing directory to {}", cli.root.display()))?;
    match cli.command {
        Command::Regen => regen(),
        Command::Serve => serve(),
        Command::Upload => upload(),
        Command::Init => init(),
    }
}

fn regen() -> Result<()> {
    ensure_blog()?;
    let config = read_config()?;
    let templates = read_templates()?;

    println!("===> Reading all posts");
    let posts = read_posts()?;
    println!("===> Regenerating blog");

    let staging = Path::new(STAGING_DIR);
    if staging.exists() {
        fs::remove_dir_all(staging).context("removing stale staging directory")?;
    }
    fs::create_dir(staging).context("creating staging directory")?;

    println!("--> Building home");
    build_home(staging, &config.site, &templates, &posts)?;
    println!("--> Building archive");
    build_archive(staging, &config.site, &templates, &posts)?;
    build_pages(staging, &config.site, &templates)?;
    println!("--> Building posts");
    build_posts(staging, &templates, &posts)?;
    println!("--> Building feeds");
    build_feeds(staging, &config.site, &posts)?;
    println!("--> Copying extra files");
    copy_extras(staging, &config.site.extrafiles)?;
    copy_tree(Path::new("static"), &staging.join("static"))?;
    println!("--> Building sitemap");
    build_sitemap(staging, &config.site.url)?;

    let output = Path::new(OUTPUT_DIR);
    if output.exists() {
        fs::remove_dir_all(output).context("removing old site directory")?;
    }
    fs::rename(staging, output).context("installing regenerated site")?;
    Ok(())
}

fn ensure_blog() -> Result<()> {
    let required_dirs = ["pages", "posts", "static", "templates"];
    if !Path::new("config.yaml").is_file()
        || required_dirs.iter().any(|path| !Path::new(path).is_dir())
    {
        bail!("not a blog: expected config.yaml, pages/, posts/, static/ and templates/");
    }
    Ok(())
}

fn read_config() -> Result<Config> {
    let yaml = fs::read_to_string("config.yaml").context("reading config.yaml")?;
    serde_yaml::from_str(&yaml).context("parsing config.yaml")
}

fn read_templates() -> Result<Templates> {
    Ok(Templates {
        default: fs::read_to_string("templates/default.tmpl")
            .context("reading templates/default.tmpl")?,
        post: fs::read_to_string("templates/post.tmpl").context("reading templates/post.tmpl")?,
    })
}

fn read_posts() -> Result<Vec<Post>> {
    let paths = fs::read_dir("posts")
        .context("reading posts directory")?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .collect::<Vec<_>>();
    let results = paths
        .par_iter()
        .map(|path| read_post(path))
        .collect::<Vec<_>>();
    let mut posts = results.into_iter().collect::<Result<Vec<_>>>()?;
    // Perl's stable sort retained directory order for posts sharing a date.
    posts.sort_by(|left, right| right.date.cmp(&left.date));
    Ok(posts)
}

fn read_post(path: &Path) -> Result<Post> {
    println!(
        "--> Reading post \"{}\"",
        path.file_name().unwrap().to_string_lossy()
    );
    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut sections = source.splitn(3, '\n');
    let title = sections
        .next()
        .ok_or_else(|| anyhow!("{} has no title", path.display()))?
        .trim_end_matches('\r')
        .to_owned();
    let date = sections
        .next()
        .ok_or_else(|| anyhow!("{} has no date", path.display()))?
        .trim_end_matches('\r')
        .to_owned();
    NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .with_context(|| format!("invalid date in {}", path.display()))?;
    let processed = process_code_blocks(sections.next().unwrap_or_default())
        .with_context(|| format!("highlighting code in {}", path.display()))?;
    let content = render_markdown(&processed)?;
    let link = format!("/{}/{}/", date.replace('-', "/"), titleify(&title));
    Ok(Post {
        title,
        date,
        content,
        link,
    })
}

fn process_code_blocks(markdown: &str) -> Result<String> {
    let mut result = String::with_capacity(markdown.len());
    let mut code = String::new();
    let mut language = String::new();
    let mut in_block = false;

    for line in markdown.lines() {
        if let Some(fence) = line.find("```") {
            if in_block {
                if language.is_empty() {
                    result.push_str("<pre><code>");
                    result.push_str(&escape_html(&code));
                    result.push_str("</code></pre>\n");
                } else {
                    result.push_str(&highlight(&language, &code)?);
                }
                code.clear();
            } else {
                language = line[fence + 3..].trim().to_owned();
            }
            in_block = !in_block;
        } else if in_block {
            code.push_str(line);
            code.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    Ok(result)
}

fn highlight(language: &str, code: &str) -> Result<String> {
    let mut child = ProcessCommand::new("pygmentize")
        .args(["-f", "html", "-l", language])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("starting pygmentize (install Pygments to highlight code)")?;
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(code.as_bytes())
        .context("writing to pygmentize")?;
    let output = child.wait_with_output().context("waiting for pygmentize")?;
    if !output.status.success() {
        bail!(
            "pygmentize failed for {language}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    String::from_utf8(output.stdout).context("pygmentize returned non-UTF-8 output")
}

fn render_markdown(markdown: &str) -> Result<String> {
    let input_length = markdown
        .as_bytes()
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(markdown.len());
    let length = c_int::try_from(input_length).context("Markdown input is too large")?;
    let _lock = DISCOUNT_LOCK
        .lock()
        .map_err(|_| anyhow!("Discount renderer lock was poisoned"))?;

    // Text::Markdown::Discount 0.18 used these calls and flags, then appended
    // one newline. Keeping that exact path preserves the old generated bytes.
    let document = unsafe { mkd_string(markdown.as_ptr().cast(), length, DISCOUNT_FLAGS) };
    if document.is_null() {
        bail!("Discount failed to read Markdown");
    }

    struct Document(*mut c_void);
    impl Drop for Document {
        fn drop(&mut self) {
            unsafe { mkd_cleanup(self.0) };
        }
    }
    let document = Document(document);

    if unsafe { mkd_compile(document.0, DISCOUNT_FLAGS) } == 0 {
        bail!("Discount failed to compile Markdown");
    }
    let mut output = std::ptr::null_mut();
    let output_length = unsafe { mkd_document(document.0, &mut output) };
    if output_length < 0 {
        bail!("Discount failed to render Markdown");
    }
    let bytes = if output_length == 0 {
        &[]
    } else {
        if output.is_null() {
            bail!("Discount returned a null output buffer");
        }
        unsafe { std::slice::from_raw_parts(output.cast::<u8>(), output_length as usize) }
    };
    let mut rendered = std::str::from_utf8(bytes)
        .context("Discount returned non-UTF-8 output")?
        .to_owned();
    rendered.push('\n');
    Ok(rendered)
}

fn build_home(root: &Path, site: &SiteConfig, templates: &Templates, posts: &[Post]) -> Result<()> {
    let mut content = String::new();
    for post in posts {
        content.push_str(&format!(
            "<a href='{}'><h2>{}</h2></a>\n<h3>{}</h3>\n",
            post.link, post.title, post.date
        ));
        let extract = post
            .content
            .lines()
            .take_while(|line| !line.contains("<!--"))
            .collect::<Vec<_>>()
            .join("\n")
            .replace("h3", "h4")
            .replace("h2", "h3")
            .replace("h1", "h2");
        content.push_str(&extract);
        content.push_str("</code></pre></ul></ol></p>\n");
        content.push_str(&format!(
            "<br/><p><a href='{}'>Read more</a></p><hr/>",
            post.link
        ));
    }
    write(
        &root.join("index.html"),
        &render_default(&templates.default, &site.title, &content)?,
    )
}

fn build_archive(
    root: &Path,
    site: &SiteConfig,
    templates: &Templates,
    posts: &[Post],
) -> Result<()> {
    let mut content = String::new();
    for post in posts {
        content.push_str(&format!(
            "<table>\n <tr>\n  <td>\n   <time datetime=\"{}\" pubdate=\"true\">{}</time>\n  </td>\n  <td>\n   <a href=\"{}\">{}</a>\n  </td>\n </tr>\n</table>\n",
            post.date, post.date, post.link, post.title
        ));
    }
    write(
        &root.join("archive.html"),
        &render_default(&templates.default, &site.title, &content)?,
    )
}

fn build_pages(root: &Path, site: &SiteConfig, templates: &Templates) -> Result<()> {
    let mut pages = fs::read_dir("pages")
        .context("reading pages directory")?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.is_file() && !path.file_name().unwrap().to_string_lossy().starts_with('.')
        })
        .collect::<Vec<_>>();
    pages.sort();
    for path in pages {
        let name = path.file_name().unwrap().to_string_lossy();
        println!("--> Building page {name}");
        let content = render_markdown(
            &fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?,
        )?;
        write(
            &root.join(format!("{name}.html")),
            &render_default(&templates.default, &site.title, &content)?,
        )?;
    }
    Ok(())
}

fn build_posts(root: &Path, templates: &Templates, posts: &[Post]) -> Result<()> {
    for post in posts {
        let path = root
            .join(post.date.replace('-', "/"))
            .join(titleify(&post.title))
            .join("index.html");
        let body = render_post(&templates.post, post);
        write(
            &path,
            &render_default(&templates.default, &post.title, &body)?,
        )?;
    }
    Ok(())
}

fn render_default(template: &str, title: &str, content: &str) -> Result<String> {
    let mut rendered = template.replace("<% $title %>", title);
    let start = rendered
        .find("<% unless ($subtemplate")
        .ok_or_else(|| anyhow!("default template has no content block"))?;
    let relative_end = rendered[start..]
        .find("} %>")
        .ok_or_else(|| anyhow!("default template content block is not closed"))?;
    let end = start + relative_end + "} %>".len();
    rendered.replace_range(start..end, content);
    Ok(rendered)
}

fn render_post(template: &str, post: &Post) -> String {
    template
        .replace("<% $title %>", &post.title)
        .replace("<% $date %>", &post.date)
        .replace("<% $content %>", &post.content)
        .replace("<% $link %>", &post.link)
}

fn build_feeds(root: &Path, site: &SiteConfig, posts: &[Post]) -> Result<()> {
    let feeds = root.join("feeds");
    fs::create_dir(&feeds).context("creating feeds directory")?;
    let mut rss = format!(
        "<rss version=\"2.0\">\n<channel>\n<title>{}</title>\n<link>{}</link>\n<description>{}</description>\n",
        site.title, site.url, site.tagline
    );
    for post in posts {
        rss.push_str(&format!(
            "<item>\n<title>{}</title>\n<link>{}{}</link>\n<description></description>\n<pubDate>{}</pubDate>\n<guid>{}</guid>\n</item>\n",
            post.title, site.url, post.link, rss_date(&post.date)?, post.link
        ));
    }
    rss.push_str("</channel></rss>");
    write(&feeds.join("all.rss.xml"), &rss)?;

    let first = posts
        .first()
        .ok_or_else(|| anyhow!("cannot build a feed with no posts"))?;
    let root_id = idify(&site.url, &format!("{}/index.html", site.url));
    let mut atom = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<feed xmlns=\"http://www.w3.org/2005/Atom\" xml:lang=\"en\">\n<title type=\"text\">{}: {}</title>\n<link rel=\"self\" href=\"{}/feeds/all.atom.xml\" />\n<link href=\"{}/index.html\" />\n<id>{}</id>\n<updated>{}</updated>\n",
        site.title,
        site.tagline,
        site.url,
        site.url,
        root_id,
        atom_date(&first.date)?
    );
    for post in posts {
        let absolute = format!("{}{}", site.url, post.link);
        let date = atom_date(&post.date)?;
        atom.push_str(&format!(
            "<entry>\n  <title type=\"text\">{}</title>\n  <link rel=\"alternate\" href=\"{}\" />\n  <id>{}</id>\n  <published>{}</published>\n  <updated>{}</updated>\n  <author>\n   <name>{}</name>\n  </author>\n  <content type=\"html\">\n{}\n</content></entry>\n",
            post.title,
            absolute,
            idify(&site.url, &absolute),
            date,
            date,
            site.author,
            escape_html(&post.content)
        ));
    }
    atom.push_str("</feed>");
    write(&feeds.join("all.atom.xml"), &atom)
}

fn rss_date(date: &str) -> Result<String> {
    Ok(NaiveDate::parse_from_str(date, "%Y-%m-%d")?
        .format("%a, %d %b %Y 12:00:00 AM -0000")
        .to_string())
}

fn atom_date(date: &str) -> Result<String> {
    Ok(format!(
        "{}T00:00:00Z",
        NaiveDate::parse_from_str(date, "%Y-%m-%d")?
    ))
}

fn idify(url: &str, input: &str) -> String {
    let trimmed = input.strip_suffix('/').unwrap_or(input);
    let input = trimmed
        .strip_prefix(url)
        .unwrap_or(trimmed)
        .replace('/', "-");
    let id_url = url
        .replace(':', "")
        .replacen("//", "/", 1)
        .replace(['/', '.'], "-");
    format!("urn:{id_url}:{input}")
}

fn build_sitemap(root: &Path, url: &str) -> Result<()> {
    let mut paths = Vec::new();
    find_html(root, &mut paths)?;
    let entries = paths
        .into_iter()
        .map(|entry| {
            let relative = entry.strip_prefix(root).expect("path is below root");
            let mut path = format!("/{}", relative.to_string_lossy());
            if path.ends_with("index.html") {
                path.truncate(path.len() - "index.html".len());
            }
            format!("{url}{path}")
        })
        .collect::<Vec<_>>();
    write(&root.join("sitemap.txt"), &(entries.join("\n") + "\n"))
}

// File::Find visits the files in a directory before recursing into the
// directories it found. Preserve that detail because sitemap order is output.
fn find_html(directory: &Path, output: &mut Vec<PathBuf>) -> Result<()> {
    let mut directories = Vec::new();
    for entry in fs::read_dir(directory)
        .with_context(|| format!("reading sitemap directory {}", directory.display()))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            directories.push(entry.path());
        } else if file_type.is_file()
            && entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "html")
        {
            output.push(entry.path());
        }
    }
    for child in directories {
        find_html(&child, output)?;
    }
    Ok(())
}

fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            '`' => output.push_str("&#96;"),
            '{' => output.push_str("&#123;"),
            '}' => output.push_str("&#125;"),
            _ => output.push(character),
        }
    }
    output
}

fn copy_extras(root: &Path, extras: &[PathBuf]) -> Result<()> {
    for source in extras {
        let name = source
            .file_name()
            .ok_or_else(|| anyhow!("invalid extra path {}", source.display()))?;
        let destination = root.join(name);
        if source.is_dir() {
            copy_tree(source, &destination)?;
        } else {
            fs::copy(source, &destination).with_context(|| {
                format!("copying {} to {}", source.display(), destination.display())
            })?;
        }
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry.with_context(|| format!("walking {}", source.display()))?;
        let relative = entry
            .path()
            .strip_prefix(source)
            .expect("walk entry is below root");
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("creating {}", target.display()))?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target).with_context(|| {
                format!("copying {} to {}", entry.path().display(), target.display())
            })?;
        }
    }
    Ok(())
}

fn write(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("writing {}", path.display()))
}

fn titleify(title: &str) -> String {
    title
        .chars()
        .map(|character| {
            if character == '-' || character.is_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .replace("--", "-")
        .trim_end_matches('-')
        .trim()
        .to_lowercase()
}

fn upload() -> Result<()> {
    let config = read_config()?;
    println!("==> Running \"{}\"", config.site.uploadcmd);
    let status = ProcessCommand::new("sh")
        .args(["-c", &config.site.uploadcmd])
        .status()
        .context("running upload command")?;
    if !status.success() {
        bail!("upload command exited with {status}");
    }
    Ok(())
}

fn init() -> Result<()> {
    println!("===> Initialising default blog");
    if fs::read_dir(".")
        .context("reading current directory")?
        .next()
        .is_some()
    {
        bail!("directory is not empty");
    }
    copy_tree(Path::new(DATA_ROOT), Path::new("."))
}

fn serve() -> Result<()> {
    let root = Path::new(OUTPUT_DIR)
        .canonicalize()
        .context("no site/ directory; run 'tau regen' first")?;
    println!("==> Serving site/ on http://localhost:8080");
    let server = Server::http("127.0.0.1:8080").map_err(|error| anyhow!(error.to_string()))?;
    for request in server.incoming_requests() {
        let relative = request
            .url()
            .split('?')
            .next()
            .unwrap_or("/")
            .trim_start_matches('/');
        let requested = if relative.is_empty() {
            root.join("index.html")
        } else {
            root.join(relative)
        };
        let path = if requested.is_dir() {
            requested.join("index.html")
        } else {
            requested
        };
        let safe = path
            .canonicalize()
            .ok()
            .filter(|path| path.starts_with(&root));
        if let Some(path) = safe.filter(|path| path.is_file()) {
            let mut file = fs::File::open(&path)?;
            let mut body = Vec::new();
            file.read_to_end(&mut body)?;
            let header = Header::from_bytes("Content-Type", mime_type(&path))
                .map_err(|_| anyhow!("invalid Content-Type header"))?;
            request.respond(Response::from_data(body).with_header(header))?;
        } else {
            request.respond(Response::empty(StatusCode(404)))?;
        }
    }
    Ok(())
}

fn mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("xml") => "application/xml; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
