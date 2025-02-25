use yazi_config::{open::Opener, popup::InputCfg};
use yazi_shared::event::Cmd;

use crate::{input::Input, tab::Tab, tasks::Tasks};

pub struct Opt {
	exec:    String,
	block:   bool,
	confirm: bool,
}

impl From<Cmd> for Opt {
	fn from(mut c: Cmd) -> Self {
		Self {
			exec:    c.take_first().unwrap_or_default(),
			block:   c.named.contains_key("block"),
			confirm: c.named.contains_key("confirm"),
		}
	}
}

impl Tab {
	pub fn shell(&self, opt: impl Into<Opt>) {
		let mut opt = opt.into() as Opt;
		let selected: Vec<_> = self.selected().into_iter().map(|f| f.url()).collect();

		tokio::spawn(async move {
			if !opt.confirm || opt.exec.is_empty() {
				let mut result = Input::_show(InputCfg::shell(opt.block).with_value(opt.exec));
				match result.recv().await {
					Some(Ok(e)) => opt.exec = e,
					_ => return,
				}
			}

			Tasks::_open(selected, Opener {
				exec:   opt.exec,
				block:  opt.block,
				orphan: false,
				desc:   Default::default(),
				for_:   None,
				spread: true,
			});
		});
	}
}
