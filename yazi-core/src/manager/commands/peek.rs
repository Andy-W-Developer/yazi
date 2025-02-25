use yazi_shared::{emit, event::Cmd, fs::Url, render, Layer};

use crate::manager::Manager;

#[derive(Debug, Default)]
pub struct Opt {
	skip:        Option<usize>,
	force:       bool,
	only_if:     Option<Url>,
	upper_bound: bool,
}

impl From<Cmd> for Opt {
	fn from(mut c: Cmd) -> Self {
		Self {
			skip:        c.take_first().and_then(|s| s.parse().ok()),
			force:       c.named.contains_key("force"),
			only_if:     c.take_name("only-if").map(Url::from),
			upper_bound: c.named.contains_key("upper-bound"),
		}
	}
}
impl From<bool> for Opt {
	fn from(force: bool) -> Self { Self { force, ..Default::default() } }
}

impl Manager {
	#[inline]
	pub fn _peek(force: bool) {
		emit!(Call(Cmd::new("peek").with_bool("force", force), Layer::Manager));
	}

	pub fn peek(&mut self, opt: impl Into<Opt>) {
		let Some(hovered) = self.hovered().cloned() else {
			return render!(self.active_mut().preview.reset());
		};

		let folder = Some(())
			.filter(|_| hovered.is_dir())
			.and_then(|_| self.active().history.get(&hovered.url))
			.map(|f| (f.offset, f.mtime));

		if !self.active().preview.same_url(&hovered.url) {
			self.active_mut().preview.skip = folder.map(|f| f.0).unwrap_or_default();
			render!(self.active_mut().preview.reset());
		}

		let opt = opt.into() as Opt;
		if matches!(opt.only_if, Some(ref u) if *u != hovered.url) {
			return;
		}

		if let Some(skip) = opt.skip {
			let preview = &mut self.active_mut().preview;
			if opt.upper_bound {
				preview.skip = preview.skip.min(skip);
			} else {
				preview.skip = skip;
			}
		}

		if hovered.is_dir() {
			self.active_mut().preview.go_folder(hovered, folder.and_then(|f| f.1), opt.force);
			return;
		}

		if let Some(mime) = self.mimetype.get(&hovered.url).cloned() {
			self.active_mut().preview.go(hovered, &mime, opt.force);
		} else {
			render!(self.active_mut().preview.reset());
		}
	}
}
