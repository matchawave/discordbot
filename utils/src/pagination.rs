use chrono::{DateTime, Duration, Utc};
use serenity::all::{CreateActionRow, CreateButton, CreateEmbed, ReactionType};

#[derive(Clone, Debug)]
pub struct Pagination {
    index: usize,
    embeds: Vec<CreateEmbed>,
    components: [CreateButton; 2],
    cooldown: DateTime<Utc>,
}

impl Pagination {
    pub fn new(id: u64, embeds: Vec<CreateEmbed>) -> Self {
        let prev_button = {
            let id = format!("page|{}|previous", id);
            CreateButton::new(id)
                .emoji(ReactionType::Unicode("◀️".to_string()))
                .style(serenity::all::ButtonStyle::Primary)
                .disabled(true)
        };
        let next_button = {
            let id = format!("page|{}|next", id);
            CreateButton::new(id)
                .emoji(ReactionType::Unicode("▶️".to_string()))
                .style(serenity::all::ButtonStyle::Primary)
        };

        Self {
            index: 0,
            embeds,
            components: [prev_button, next_button],
            cooldown: Utc::now() + Duration::minutes(3),
        }
    }

    pub fn next_page(&mut self) -> Option<(CreateEmbed, CreateActionRow)> {
        let prev_index = self.index;
        if prev_index + 1 >= self.embeds.len() {
            return None;
        }
        let index = self.index + 1;
        let embed = &self.embeds[index];
        self.components[0] = self.components[0].clone().disabled(false);
        if index + 1 >= self.embeds.len() {
            self.components[1] = self.components[1].clone().disabled(true);
        }
        self.index = index;
        self.cooldown = Utc::now() + Duration::minutes(3);
        let components = CreateActionRow::Buttons(self.components.to_vec());
        Some((embed.clone(), components))
    }

    pub fn prev_page(&mut self) -> Option<(CreateEmbed, CreateActionRow)> {
        let prev_index = self.index;
        if prev_index == 0 {
            return None;
        }
        let index = self.index - 1;
        let embed = &self.embeds[index];
        self.components[1] = self.components[1].clone().disabled(false);
        if index == 0 {
            self.components[0] = self.components[0].clone().disabled(true);
        }
        self.index = index;
        self.cooldown = Utc::now() + Duration::minutes(3);
        let components = CreateActionRow::Buttons(self.components.to_vec());
        Some((embed.clone(), components))
    }

    pub fn current(&self) -> (CreateEmbed, CreateActionRow) {
        let embed = &self.embeds[self.index];
        let components = CreateActionRow::Buttons(self.components.to_vec());
        (embed.clone(), components)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.cooldown
    }
}

pub enum PaginationAction {
    Next,
    Previous,
}

impl From<&str> for PaginationAction {
    fn from(s: &str) -> Self {
        match s {
            "next" => PaginationAction::Next,
            "previous" => PaginationAction::Previous,
            _ => PaginationAction::Next, // Default to Next if unknown
        }
    }
}
