use crate::components::default_block;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::Frame;
use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
