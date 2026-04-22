use std::time::Instant;

use ratatui::widgets::TableState;

use crate::model::{PrDetail, PrSummary};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Detail(u32),
}

pub struct App {
    pub repo: String,
    pub prs: Vec<PrSummary>,
    pub table_state: TableState,
    pub mode: ViewMode,
    pub loading_list: bool,
    pub loading_detail: bool,
    pub detail: Option<PrDetail>,
    pub last_refresh: Option<Instant>,
    pub last_error: Option<String>,
    pub show_help: bool,
    pub should_quit: bool,
    pub refresh_interval_secs: u64,
    pub auto_refresh: bool,
    pub limit: u32,
}

impl App {
    pub fn new(repo: String, refresh_interval_secs: u64, auto_refresh: bool, limit: u32) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        App {
            repo,
            prs: Vec::new(),
            table_state,
            mode: ViewMode::List,
            loading_list: true,
            loading_detail: false,
            detail: None,
            last_refresh: None,
            last_error: None,
            show_help: false,
            should_quit: false,
            refresh_interval_secs,
            auto_refresh,
            limit,
        }
    }

    pub fn selected_pr(&self) -> Option<&PrSummary> {
        let idx = self.table_state.selected()?;
        self.prs.get(idx)
    }

    pub fn select_next(&mut self) {
        if self.prs.is_empty() {
            return;
        }
        let new = match self.table_state.selected() {
            Some(i) if i + 1 < self.prs.len() => i + 1,
            Some(i) => i,
            None => 0,
        };
        self.table_state.select(Some(new));
    }

    pub fn select_prev(&mut self) {
        if self.prs.is_empty() {
            return;
        }
        let new = match self.table_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.table_state.select(Some(new));
    }

    pub fn select_first(&mut self) {
        if !self.prs.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if !self.prs.is_empty() {
            self.table_state.select(Some(self.prs.len() - 1));
        }
    }

    pub fn apply_prs(&mut self, prs: Vec<PrSummary>) {
        let keep_number = self.selected_pr().map(|p| p.number);
        self.prs = prs;
        if self.prs.is_empty() {
            self.table_state.select(None);
        } else if let Some(n) = keep_number {
            let idx = self.prs.iter().position(|p| p.number == n).unwrap_or(0);
            self.table_state.select(Some(idx));
        } else {
            self.table_state.select(Some(0));
        }
        self.loading_list = false;
        self.last_refresh = Some(Instant::now());
        self.last_error = None;
    }

    pub fn apply_list_error(&mut self, err: String) {
        self.loading_list = false;
        self.last_error = Some(err);
    }

    pub fn apply_detail(&mut self, detail: PrDetail) {
        self.loading_detail = false;
        self.detail = Some(detail);
    }

    pub fn apply_detail_error(&mut self, err: String) {
        self.loading_detail = false;
        self.last_error = Some(err);
    }

    pub fn enter_detail(&mut self) {
        if let Some(pr) = self.selected_pr() {
            self.mode = ViewMode::Detail(pr.number);
            self.detail = None;
            self.loading_detail = true;
        }
    }

    pub fn back_to_list(&mut self) {
        self.mode = ViewMode::List;
        self.detail = None;
        self.loading_detail = false;
    }

    pub fn needs_auto_refresh(&self) -> bool {
        if !self.auto_refresh || self.loading_list {
            return false;
        }
        match self.last_refresh {
            None => false,
            Some(t) => t.elapsed().as_secs() >= self.refresh_interval_secs,
        }
    }
}
