#![doc(html_favicon_url = "https://watchexec.github.io/logo:command-group.svg")]
#![doc(html_logo_url = "https://watchexec.github.io/logo:command-group.svg")]
// #![warn(missing_docs)]

// #[cfg(feature = "std")]
// pub mod std;

#[cfg(feature = "tokio1")]
pub mod tokio;

/// Internal memoization of the exit status of a child process.
#[derive(Debug)]
pub(crate) enum ChildExitStatus {
	Running,
	Exited(std::process::ExitStatus),
}
