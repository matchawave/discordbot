use std::sync::Arc;

use serenity::{
    all::{
        Colour, CommandOptionType, CommandType, Context, CreateCommandOption, CreateEmbed,
        CreateEmbedAuthor, FormattedTimestamp, FormattedTimestampStyle, Guild, GuildChannel,
        Member, Mentionable, Permissions, Role,
    },
    async_trait,
};

use utils::{
    CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand,
    PERMISSION_PRIORITY, UserType,
};

use crate::commands::command_member_target;

const COMMAND_NAME: &str = "info";
const COMMAND_DESCRIPTION: &str = "Get information about a user";

pub struct Command;

pub fn command() -> CommandTemplate {
    let user_option = CreateCommandOption::new(
        CommandOptionType::User,
        "user",
        "The user to get the banner of",
    );
    (
        ICommand::new(
            COMMAND_NAME.to_string(),
            COMMAND_DESCRIPTION.to_string(),
            CommandType::ChatInput,
            vec![user_option],
            vec![],
        ),
        Arc::new(Command),
    )
}

#[async_trait]
impl CommandTrait for Command {
    async fn execute<'a>(
        &self,
        ctx: &'a Context,
        user: UserType,
        location: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((guild, _)) = location else {
            return Err("This command can only be used in a server.".to_string());
        };
        let member = match command_member_target(ctx, &args, &guild).await {
            Some(m) => m.clone(),
            None => match user {
                UserType::Member(m) => m.clone(),
                UserType::User(_) => {
                    return Err(
                        "You must specify a user when using this command outside of a server."
                            .to_string(),
                    );
                }
            },
        };
        let mut fields = vec![];
        let mut roles = member
            .roles
            .iter()
            .map(|r| guild.roles.get(r).cloned().expect("Role not found"))
            .collect::<Vec<_>>();
        roles.sort_by(|a, b| b.position.cmp(&a.position)); // Sort roles by position, highest first

        let author_section =
            CreateEmbedAuthor::new(format!("{} ({})", member.user.name, member.user.id));
        fields.push(("Dates", get_dates(&member), false));

        let (roles_title, roles_content) = get_roles(&mut roles);
        fields.push((roles_title.as_str(), roles_content, false));

        let permissions = guild.member_permissions(&member);
        fields.push(get_permissions(&permissions));

        let avatar_url = member
            .user
            .avatar_url()
            .unwrap_or(member.user.default_avatar_url());

        let embed = CreateEmbed::default()
            .author(author_section)
            .title(format!(
                "Information about {}{}",
                member.user.display_name(),
                { if member.user.bot { " 🤖" } else { "" } }
            ))
            .fields(fields)
            .thumbnail(avatar_url)
            .color(Colour::BLITZ_BLUE);

        Ok(Some(CommandResponse::new_embeds(vec![embed])))
    }
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}

fn get_permissions<'a>(permissions: &Permissions) -> (&'a str, String, bool) {
    let mut permissions_list = vec![];
    let mut count = 0;

    for perm in PERMISSION_PRIORITY.iter() {
        if permissions.contains(*perm)
            && let Some(name) = perm.get_permission_names().into_iter().next()
        {
            permissions_list.push(name.to_string());
            count += 1;
        }
        if count >= 3 {
            break;
        }
    }

    (
        "Key Permissions",
        if permissions_list.is_empty() {
            "None".to_string()
        } else {
            permissions_list.join(", ")
        },
        false,
    )
}

fn get_dates(member: &Member) -> String {
    let joined_at_string = match member.joined_at {
        Some(d) => format!(
            "{} ({})",
            d.format("%Y-%m-%d %H:%M:%S"),
            FormattedTimestamp::new(d, Some(FormattedTimestampStyle::RelativeTime))
        ),
        None => "Unknown".to_string(),
    };
    let created_at_string = {
        let created_at = member.user.created_at();
        format!(
            "{} ({})",
            created_at.format("%Y-%m-%d %H:%M:%S"),
            FormattedTimestamp::new(created_at, Some(FormattedTimestampStyle::RelativeTime))
        )
    };

    format!(
        "Joined: {}\nCreated: {}",
        joined_at_string, created_at_string
    )
}

fn get_roles(roles: &mut Vec<Role>) -> (String, String) {
    (
        format!("Roles ({})", roles.len()),
        if roles.is_empty() {
            "None".to_string()
        } else {
            let length = roles.len();
            if length > 7 {
                roles.truncate(7);
                format!("{} and {} more...", write_roles(roles), length - 7)
            } else {
                write_roles(roles)
            }
        },
    )
}

fn write_roles(roles: &[Role]) -> String {
    roles
        .iter()
        .map(|r| r.mention().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
