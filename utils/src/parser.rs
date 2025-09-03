use std::collections::HashMap;

use regex::Regex;
use serenity::all::{
    AfkTimeout, ChannelType, Context, FormattedTimestamp, FormattedTimestampStyle, Guild,
    GuildChannel, Member, PremiumTier, Role, Timestamp,
};

const NOT_AVAILABLE: &str = "`n/a`";
#[derive(Debug, Clone)]
pub struct BotStringParser<'a> {
    ctx: &'a Context,
    guild: &'a Guild,
    channel: &'a GuildChannel,
    member: &'a Member,

    cache: HashMap<String, ParserCache>,
    roles: Vec<&'a Role>,
    member_pos: Option<usize>,
}

type ParserCache = HashMap<String, Option<String>>;

impl<'a> BotStringParser<'a> {
    pub fn new(
        ctx: &'a Context,
        guild: &'a Guild,
        channel: &'a GuildChannel,
        member: &'a Member,
    ) -> Self {
        Self {
            ctx,
            member,
            channel,
            guild,
            cache: HashMap::new(),
            roles: Vec::new(),
            member_pos: None,
        }
    }

    pub fn render(&mut self, input: &str) -> String {
        let re = Regex::new(r"\{([^}]+)\}").unwrap();

        re.replace_all(input, |caps: &regex::Captures| {
            let path = &caps[1];
            let mut parts = path.split('.');

            match (parts.next(), parts.next()) {
                (Some(section), Some(key)) => {
                    let mut cache = self
                        .cache
                        .get(section)
                        .cloned()
                        .unwrap_or(ParserCache::new());

                    let value = cache.get(key).cloned();

                    if (key.contains("role") || key.contains("color")) && self.roles.is_empty() {
                        let roles = self.member.roles.clone();
                        let mut roles = roles
                            .iter()
                            .filter_map(|r| self.guild.roles.get(r))
                            .collect::<Vec<_>>();
                        roles.sort_by(|a, b| b.position.cmp(&a.position));

                        self.roles = roles;
                    }

                    if key.contains("join") && self.member_pos.is_none() {
                        let mut members = self.guild.members.values().collect::<Vec<&Member>>();
                        members.sort_by(|a, b| a.joined_at.cmp(&b.joined_at));
                        let index = members
                            .iter()
                            .position(|m| m.user.id == self.member.user.id);
                        self.member_pos = index;
                    }

                    let output = match section {
                        "user" => value.unwrap_or_else(|| Some(self.handle_user(key, &mut cache))),
                        "guild" => {
                            value.unwrap_or_else(|| Some(self.handle_guild(key, &mut cache)))
                        }
                        "channel" => {
                            value.unwrap_or_else(|| Some(self.handle_channel(key, &mut cache)))
                        }
                        "date" => value.unwrap_or_else(|| Some(self.handle_date(key, &mut cache))),
                        "time" => value.unwrap_or_else(|| Some(self.handle_time(key, &mut cache))),
                        "level" => {
                            value.unwrap_or_else(|| Some(self.handle_level(key, &mut cache)))
                        }
                        _ => return NOT_AVAILABLE.to_string(),
                    };
                    self.cache.insert(section.to_string(), cache);
                    output.unwrap_or_else(|| NOT_AVAILABLE.to_string())
                }
                (Some(section), None) => match section {
                    "user" => format!("<@{}>", self.member.user.id),
                    "channel" => format!("<#{}>", self.channel.id),
                    _ => NOT_AVAILABLE.to_string(),
                },
                (_, _) => NOT_AVAILABLE.to_string(),
            }
        })
        .to_string()
    }
    fn handle_user(&self, key: &'a str, cache: &mut ParserCache) -> String {
        let member = self.member;
        let roles = &self.roles;

        let value = match key {
            "id" => Some(member.user.id.to_string()),
            "name" => Some(member.user.name.clone()),
            "tag" => member.user.discriminator.map(|d| d.to_string()),
            "avatar" => member.user.avatar_url(),
            "guild_avatar" => member.avatar_url(),
            "joined_at" => member.joined_at.map(|ts| {
                FormattedTimestamp::new(ts, Some(FormattedTimestampStyle::ShortDateTime))
                    .to_string()
            }),
            "joined_at_timestamp" => member.joined_at.map(|ts| {
                FormattedTimestamp::new(ts, Some(FormattedTimestampStyle::ShortDateTime))
                    .to_string()
            }),
            "created_at" => Some(
                FormattedTimestamp::new(
                    member.user.created_at(),
                    Some(FormattedTimestampStyle::ShortDateTime),
                )
                .to_string(),
            ),
            "created_at_timestamp" => Some(member.user.created_at().to_string()),
            "display_name" => Some(member.display_name().to_string()),
            "boost" => Some(
                if member.premium_since.is_some() {
                    "Yes"
                } else {
                    "No"
                }
                .to_string(),
            ),
            "boost_since" => member.premium_since.map(|ts| ts.to_string()),
            "boost_since_timestamp" => member.premium_since.map(|ts| ts.to_string()),
            "color" => roles.first().map(|r| r.colour.hex()),
            "top_role" => roles.first().map(|r| r.name.clone()),
            "role_list" => Some(
                roles
                    .iter()
                    .map(|r| format!("<@&{}>", r.id))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "role_text_list" => Some(
                roles
                    .iter()
                    .map(|r| r.id.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "bot" => Some(if member.user.bot { "Yes" } else { "No" }.to_string()),
            "join_position" => self.member_pos.map(|p| (p + 1).to_string()),
            "join_position_suffix" => self.member_pos.map(super::position_suffix),
            _ => None,
        };
        cache.insert(key.to_string(), value.clone());
        value.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string())
    }

    fn handle_channel(&self, key: &str, cache: &mut ParserCache) -> String {
        let channel = self.channel;
        let value = match key {
            "name" => Some(channel.name.clone()),
            "id" => Some(channel.id.to_string()),
            "topic" => channel.topic.clone(),
            "type" => Some(channel_type(channel.kind)),
            "category_id" => channel.parent_id.map(|id| id.to_string()),
            "category" => channel
                .parent_id
                .and_then(|id| self.guild.channels.get(&id))
                .map(|c| c.name.clone()),
            "position" => Some(channel.position.to_string()),
            "slowmode" => Some(
                channel
                    .rate_limit_per_user
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "0".to_string()),
            ),
            _ => None,
        };
        cache.insert(key.to_string(), value.clone());
        value.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string())
    }

    fn handle_guild(&self, key: &str, cache: &mut ParserCache) -> String {
        let guild = self.guild;
        let value = match key {
            "id" => Some(guild.id.to_string()),
            "name" => Some(guild.name.clone()),
            "count" => Some(guild.member_count.to_string()),
            "shard" => Some(guild.shard_id(self.ctx).to_string()),
            "owner_id" => Some(guild.owner_id.to_string()),
            "created_at" => Some(
                FormattedTimestamp::new(
                    guild.id.created_at(),
                    Some(FormattedTimestampStyle::ShortDateTime),
                )
                .to_string(),
            ),
            "created_at_timestamp" => Some(guild.id.created_at().to_string()),
            "emoji_count" => Some(guild.emojis.len().to_string()),
            "role_count" => Some(guild.roles.len().to_string()),
            "boost_count" => guild
                .premium_subscription_count
                .map(|count| count.to_string()),
            "boost_tier" => Some(premium_tier(guild.premium_tier)),
            "preferred_locale" => Some(guild.preferred_locale.clone()),
            "key_features" => Some(guild.features.join(", ")),
            "icon" => guild.icon_url().map(|url| url.to_string()),
            "banner" => guild.banner_url().map(|url| url.to_string()),
            "splash" => guild.splash_url().map(|url| url.to_string()),
            "max_presences" => guild.max_presences.map(|count| count.to_string()),
            "max_members" => guild.max_members.map(|count| count.to_string()),
            "max_video_channel_users" => {
                guild.max_video_channel_users.map(|count| count.to_string())
            }
            "afk_timeout" => guild
                .afk_metadata
                .as_ref()
                .map(|meta| afk_timeout(meta.afk_timeout)),
            "afk_channel" => guild
                .afk_metadata
                .as_ref()
                .map(|meta| meta.afk_channel_id.to_string()),
            "channels" => Some(
                guild
                    .channels
                    .keys()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "channels_count" => Some(guild.channels.len().to_string()),
            "text_channels" => Some(
                guild
                    .channels
                    .iter()
                    .filter(|c| matches!(c.1.kind, ChannelType::Text | ChannelType::News))
                    .map(|c| c.1.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "text_channels_count" => Some(
                guild
                    .channels
                    .iter()
                    .filter(|c| matches!(c.1.kind, ChannelType::Text | ChannelType::News))
                    .count()
                    .to_string(),
            ),
            "voice_channels" => Some(
                guild
                    .channels
                    .iter()
                    .filter(|c| matches!(c.1.kind, ChannelType::Voice))
                    .map(|c| c.1.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "voice_channels_count" => Some(
                guild
                    .channels
                    .iter()
                    .filter(|c| matches!(c.1.kind, ChannelType::Voice))
                    .count()
                    .to_string(),
            ),
            "category_channels" => Some(
                guild
                    .channels
                    .iter()
                    .filter(|c| matches!(c.1.kind, ChannelType::Category))
                    .map(|c| c.1.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "category_channels_count" => Some(
                guild
                    .channels
                    .iter()
                    .filter(|c| matches!(c.1.kind, ChannelType::Category))
                    .count()
                    .to_string(),
            ),
            "vanity" => guild
                .vanity_url_code
                .as_ref()
                .map(|code| format!("https://discord.gg/{}", code)),
            _ => None,
        };
        cache.insert(key.to_string(), value.clone());
        value.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string())
    }

    fn handle_date(&self, key: &str, cache: &mut ParserCache) -> String {
        let now = chrono::Utc::now();
        let value = match key {
            "now" => Some(
                FormattedTimestamp::new(Timestamp::now(), Some(FormattedTimestampStyle::LongDate))
                    .to_string(),
            ),
            "now_text" => Some(now.format("%B %d, %Y").to_string()),
            "now_short" => Some(
                FormattedTimestamp::new(Timestamp::now(), Some(FormattedTimestampStyle::ShortDate))
                    .to_string(),
            ),
            "now_short_text" => Some(now.format("%Y-%m-%d").to_string()),
            _ => None,
        };
        cache.insert(key.to_string(), value.clone());
        value.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string())
    }

    fn handle_time(&self, key: &str, cache: &mut ParserCache) -> String {
        let now = chrono::Utc::now();
        let value = match key {
            "now" => Some(
                FormattedTimestamp::new(Timestamp::now(), Some(FormattedTimestampStyle::LongTime))
                    .to_string(),
            ),
            "now_text" => Some(now.format("%I:%M:%S %p").to_string()),
            "now_military_text" => Some(now.format("%H:%M").to_string()),
            "now_short" => Some(
                FormattedTimestamp::new(Timestamp::now(), Some(FormattedTimestampStyle::ShortTime))
                    .to_string(),
            ),
            "now_short_text" => Some(now.format("%Y-%m-%d").to_string()),
            "now_short_military_text" => Some(now.format("%H:%M:%S").to_string()),
            _ => None,
        };
        cache.insert(key.to_string(), value.clone());
        value.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string())
    }

    fn handle_level(&self, key: &str, cache: &mut ParserCache) -> String {
        if let Some(value) = cache.get(key) {
            return value.clone().unwrap_or_else(|| "`n/a`".to_string());
        }

        "`n/a`".to_string()
    }

    pub fn guild(&self) -> &Guild {
        self.guild
    }

    pub fn member(&self) -> &Member {
        self.member
    }

    pub fn channel(&self) -> &GuildChannel {
        self.channel
    }
}

fn premium_tier(tier: PremiumTier) -> String {
    match tier {
        PremiumTier::Tier0 => "No Level".to_string(),
        PremiumTier::Tier1 => "Tier 1".to_string(),
        PremiumTier::Tier2 => "Tier 2".to_string(),
        PremiumTier::Tier3 => "Tier 3".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn afk_timeout(timeout: AfkTimeout) -> String {
    match timeout {
        AfkTimeout::OneMinute => "1 Minute",
        AfkTimeout::FiveMinutes => "5 Minutes",
        AfkTimeout::FifteenMinutes => "15 Minutes",
        AfkTimeout::ThirtyMinutes => "30 Minutes",
        AfkTimeout::OneHour => "1 Hour",
        _ => "Unknown",
    }
    .to_string()
}

fn channel_type(type_: ChannelType) -> String {
    match type_ {
        ChannelType::Text => "Text",
        ChannelType::Voice => "Voice",
        ChannelType::Category => "Category",
        ChannelType::News => "News",
        ChannelType::Stage => "Stage",
        ChannelType::Directory => "Directory",
        ChannelType::Forum => "Forum",
        _ => "Unknown",
    }
    .to_string()
}
