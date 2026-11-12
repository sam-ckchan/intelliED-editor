//! "rust-analyzer is not installed — Install": offers a one-click install when a
//! language server with a known install command reports it is not installed, then
//! restarts it so code insight starts without reopening the file.

use std::time::Duration;

use gpui::App;
use gpui::AppContext as _;
use gpui::Global;
use language::BinaryStatus;
use language::LanguageServerStatusUpdate;
use lsp::LanguageServerName;
use lsp::LanguageServerSelector;
use project::LspStoreEvent;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use ui::Color;
use ui::IconName;
use ui::prelude::FluentBuilder as _;
use util::command::Stdio;
use util::command::new_command;
use workspace::Workspace;
use workspace::notifications::NotificationId;
use workspace::notifications::simple_message_notification::MessageNotification;

#[derive(Debug, PartialEq)]
struct InstallMethod {
    server: &'static str,
    languages: &'static str,
    /// The executable the adapter looks for.
    binary: &'static str,
    install: &'static [&'static str],
}

/// Only single user-level commands. Servers resolved from the project (tsgo, biome)
/// are left out so nothing is installed into the user's project. Python and Node
/// servers only report not installed when `uvx` / `npx` are missing too, so they
/// install through Homebrew rather than uv / npm.
const INSTALL_METHODS: &[InstallMethod] = &[
    InstallMethod {
        server: "rust-analyzer",
        languages: "Rust",
        binary: "rust-analyzer",
        install: &["rustup", "component", "add", "rust-analyzer"],
    },
    InstallMethod { server: "gopls", languages: "Go", binary: "gopls", install: &["brew", "install", "gopls"] },
    InstallMethod {
        server: "terraform-ls",
        languages: "Terraform",
        binary: "terraform-ls",
        install: &["brew", "install", "terraform-ls"],
    },
    InstallMethod { server: "ruff", languages: "Python", binary: "ruff", install: &["brew", "install", "ruff"] },
    InstallMethod { server: "ty", languages: "Python", binary: "ty", install: &["brew", "install", "ty"] },
    InstallMethod {
        server: "vscode-html-language-server",
        languages: "HTML",
        binary: "vscode-html-language-server",
        install: &["brew", "install", "vscode-langservers-extracted"],
    },
    InstallMethod {
        server: "vscode-css-language-server",
        languages: "CSS",
        binary: "vscode-css-language-server",
        install: &["brew", "install", "vscode-langservers-extracted"],
    },
    InstallMethod {
        server: "json-language-server",
        languages: "JSON, JSONC",
        binary: "vscode-json-language-server",
        install: &["brew", "install", "vscode-langservers-extracted"],
    },
    InstallMethod {
        server: "yaml-language-server",
        languages: "YAML",
        binary: "yaml-language-server",
        install: &["brew", "install", "yaml-language-server"],
    },
    InstallMethod {
        server: "bash-language-server",
        languages: "Shell Script",
        binary: "bash-language-server",
        install: &["brew", "install", "bash-language-server"],
    },
    InstallMethod { server: "tombi", languages: "TOML", binary: "tombi", install: &["brew", "install", "tombi"] },
];

const SUCCESS_DISMISS_DELAY: Duration = Duration::from_secs(5);
const FAILURE_OUTPUT_LINES: usize = 10;

fn install_method(server: &str) -> Option<&'static InstallMethod> {
    INSTALL_METHODS.iter().find(|method| method.server == server)
}

/// Servers installed by the same command (vscode-langservers-extracted ships three).
fn servers_installed_by(install: &[&str]) -> Vec<LanguageServerName> {
    INSTALL_METHODS
        .iter()
        .filter(|method| method.install == install)
        .map(|method| LanguageServerName::new_static(method.server))
        .collect()
}

fn prompt_message(method: &InstallMethod) -> String {
    format!(
        "Code insight for {} needs {}. Install it via {}?",
        method.languages,
        method.binary,
        method.install.join(" ")
    )
}

/// The last lines of a failed command's output, where the reason usually is.
fn failure_summary(stdout: &str, stderr: &str) -> String {
    let output = if stderr.trim().is_empty() { stdout } else { stderr };
    let lines = output.trim().lines().collect::<Vec<_>>();
    lines[lines.len().saturating_sub(FAILURE_OUTPUT_LINES)..].join("\n")
}

/// Install commands already offered this session, shared by all windows.
#[derive(Default)]
struct OfferedInstalls(FxHashSet<&'static [&'static str]>);

impl Global for OfferedInstalls {}

/// Marks the method's install command as offered; false if it already was.
fn take_offer(offered: &mut OfferedInstalls, method: &'static InstallMethod) -> bool {
    offered.0.insert(method.install)
}

pub(crate) fn init(cx: &App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        let lsp_store = workspace.project().read(cx).lsp_store();
        cx.subscribe(&lsp_store, |workspace, _, event, cx| {
            if let LspStoreEvent::LanguageServerStatus {
                name: Some(name),
                status: LanguageServerStatusUpdate::Binary(BinaryStatus::NotInstalled { .. }),
                ..
            } = event
                && let Some(method) = install_method(name.as_ref())
                && take_offer(cx.default_global::<OfferedInstalls>(), method)
            {
                show_prompt(workspace, method, cx);
            }
        })
        .detach();
    })
    .detach();
}

fn notification_id(method: &InstallMethod) -> NotificationId {
    NotificationId::named(format!("lsp-install-{}", method.server).into())
}

fn show_prompt(workspace: &mut Workspace, method: &'static InstallMethod, cx: &mut gpui::Context<Workspace>) {
    let workspace_handle = cx.weak_entity();
    // The prompt and its progress use separate ids: clicking Install dismisses the
    // prompt by id after the click handler has shown the progress notification.
    let id = NotificationId::named(format!("lsp-install-prompt-{}", method.server).into());
    workspace.show_notification(id, cx, |cx| {
        cx.new(|cx| {
            MessageNotification::new(prompt_message(method), cx)
                .with_title(format!("{} is not installed", method.server))
                .content_icon(IconName::Warning, Color::Warning)
                .primary_message("Install")
                .primary_icon(IconName::Download)
                .primary_on_click(move |_, cx| {
                    workspace_handle.update(cx, |workspace, cx| install(workspace, method, cx)).ok();
                })
        })
    });
}

fn install(workspace: &mut Workspace, method: &'static InstallMethod, cx: &mut gpui::Context<Workspace>) {
    let command = method.install.join(" ");
    let id = notification_id(method);
    workspace.show_notification(id.clone(), cx, |cx| {
        cx.new(|cx| {
            MessageNotification::new(format!("Running {command}…"), cx)
                .with_title(format!("Installing {}…", method.server))
                .content_icon(IconName::ArrowCircle, Color::Muted)
                .show_suppress_button(false)
        })
    });

    // Run in the shell environment of the project, so Homebrew and rustup are on PATH
    // even when the app was started from Finder.
    let project = workspace.project().read(cx);
    let worktree = project.visible_worktrees(cx).next();
    let environment = project.environment().clone();
    let environment = worktree
        .map(|worktree| environment.update(cx, |environment, cx| environment.worktree_environment(worktree, cx)));
    cx.spawn(async move |workspace, cx| {
        let env = match environment {
            Some(task) => task.await.unwrap_or_default(),
            None => FxHashMap::default(),
        };
        let output = cx
            .background_spawn(async move {
                new_command(method.install[0])
                    .args(&method.install[1..])
                    .envs(env)
                    .current_dir(util::paths::home_dir())
                    .stdin(Stdio::null())
                    .output()
                    .await
            })
            .await;
        let error = match output {
            Ok(output) if output.status.success() => None,
            Ok(output) => Some(failure_summary(
                &String::from_utf8_lossy(&output.stdout),
                &String::from_utf8_lossy(&output.stderr),
            )),
            Err(err) => Some(format!("{err}")),
        };
        workspace.update(cx, |workspace, cx| match error {
            None => {
                log::info!("installed {} with `{command}`", method.server);
                restart_servers(workspace, &servers_installed_by(method.install), cx);
                workspace.show_notification(id.clone(), cx, |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(format!("Restarting {}.", method.server), cx)
                            .with_title(format!("{} installed", method.server))
                            .content_icon(IconName::Check, Color::Success)
                            .show_suppress_button(false)
                    })
                });
                cx.spawn(async move |workspace, cx| {
                    cx.background_executor().timer(SUCCESS_DISMISS_DELAY).await;
                    workspace.update(cx, |workspace, cx| workspace.dismiss_notification(&id, cx)).ok();
                })
                .detach();
            }
            Some(error) => {
                log::error!("failed to install {} with `{command}`: {error}", method.server);
                workspace.show_notification(id, cx, |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(format!("{command} failed"), cx)
                            .with_title(format!("Failed to install {}", method.server))
                            .content_icon(IconName::Warning, Color::Error)
                            .when(!error.is_empty(), |this| this.secondary_content(error.clone()))
                            .copy_text(error)
                            .show_suppress_button(false)
                    })
                });
            }
        })
    })
    .detach();
}

/// Restarts the named servers for every open buffer; the name selectors limit the
/// restart to them, as the LSP menu does for a stopped server.
fn restart_servers(workspace: &Workspace, servers: &[LanguageServerName], cx: &mut App) {
    let lsp_store = workspace.project().read(cx).lsp_store();
    let buffers = workspace.project().read(cx).buffer_store().read(cx).buffers().collect();
    let selectors = servers.iter().cloned().map(LanguageServerSelector::Name).collect();
    lsp_store.update(cx, |lsp_store, cx| lsp_store.restart_language_servers_for_buffers(buffers, selectors, cx));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_install_method_by_server_name() {
        assert_eq!(install_method("rust-analyzer").unwrap().install, ["rustup", "component", "add", "rust-analyzer"]);
        assert_eq!(install_method("ruff").unwrap().install, ["brew", "install", "ruff"]);
        assert!(install_method("tsgo").is_none());
        assert!(install_method("biome").is_none());
    }

    #[test]
    fn install_commands_are_user_level_installers() {
        for method in INSTALL_METHODS {
            assert!(matches!(method.install, ["rustup", "component", "add", _] | ["brew", "install", _]), "{method:?}");
        }
    }

    #[test]
    fn groups_servers_sharing_an_install_command() {
        let names = servers_installed_by(install_method("vscode-css-language-server").unwrap().install);
        assert_eq!(
            names.iter().map(|name| name.0.as_ref()).collect::<Vec<&str>>(),
            ["vscode-html-language-server", "vscode-css-language-server", "json-language-server"]
        );
        assert_eq!(servers_installed_by(install_method("gopls").unwrap().install).len(), 1);
    }

    #[test]
    fn prompt_names_languages_binary_and_command() {
        assert_eq!(
            prompt_message(install_method("json-language-server").unwrap()),
            "Code insight for JSON, JSONC needs vscode-json-language-server. Install it via brew \
             install vscode-langservers-extracted?"
        );
    }

    #[test]
    fn failure_summary_prefers_stderr_and_keeps_last_lines() {
        assert_eq!(failure_summary("out\n", "  \n"), "out");
        assert_eq!(failure_summary("out", "error: no formula\n"), "error: no formula");
        let long = (1..=15).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
        assert_eq!(failure_summary("", &long), "6\n7\n8\n9\n10\n11\n12\n13\n14\n15");
    }

    #[test]
    fn offers_each_install_command_once() {
        let mut offered = OfferedInstalls::default();
        let html = install_method("vscode-html-language-server").unwrap();
        assert!(take_offer(&mut offered, html));
        assert!(!take_offer(&mut offered, html));
        assert!(!take_offer(&mut offered, install_method("vscode-css-language-server").unwrap()));
        assert!(take_offer(&mut offered, install_method("gopls").unwrap()));
    }
}
